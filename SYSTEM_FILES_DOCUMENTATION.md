# Dependency Resolution System: Files and Scripts

## Core Binaries (src/bin/)

### `create_name_index.rs`
- **Purpose**: Build comprehensive name index from codebase
- **Input**: output2/ directory (wrapped crates)
- **Output**: `name_index.json` (141,578+ names)
- **Key Code**: Enhanced syn visitor extracts names from macros, attributes, cfg blocks

### `recursive_resolver.rs` 
- **Purpose**: Discover all transitive dependencies from rustc functions
- **Input**: Initial rustc terms + `name_index.json`
- **Output**: `recursive_dependencies.json` (12,511+ dependencies)
- **Achievement**: 100% resolution rate vs 3.1% with basic parsing

### `generate_import_macros.rs`
- **Purpose**: Generate callable macros for each dependency
- **Input**: `recursive_dependencies.json`
- **Output**: `import_macros.rs` (12,511+ macros)
- **Key Features**: Safe name generation, correct file paths

### `generate_dependency_macros.rs`
- **Purpose**: Create execution orchestration macros
- **Input**: `recursive_dependencies.json` 
- **Output**: `dependency_macros.rs`
- **Contains**: `call_<name>!()` and `execute_all_deps!()` macros

### `test_simple.rs`
- **Purpose**: Test compilation with generated macros
- **Uses**: `mkbin!()` macro to load dependencies
- **Current Issue**: Only loads 11 of 12,511 dependencies

## Generated Files

### `name_index.json`
- **Size**: 141,578+ indexed names
- **Structure**: `{"name": ["file1.rs", "file2.rs", ...]}`
- **Purpose**: O(1) lookup for dependency resolution

### `recursive_dependencies.json`
- **Size**: 12,511+ resolved dependencies
- **Structure**: Array of dependency names
- **Coverage**: Complete transitive closure from rustc functions

### `import_macros.rs`
- **Size**: 12,511+ macro definitions
- **Pattern**: 
  ```rust
  macro_rules! import_<safe_name> {
      () => { include!("../../output2/<crate>/src/decls/<file>.rs"); };
  }
  ```

### `dependency_macros.rs`
- **Contains**: Execution macros for all dependencies
- **Key Macro**: `execute_all_deps!()` calls all 12,511 dependencies

### `src/generated_mkbin.rs`
- **Current State**: Only 11 macro calls
- **Issue**: Missing 12,500+ dependencies
- **Pattern**:
  ```rust
  mod <name> {
      println!("Module <name> loaded");
      import_<name>!();
  }
  ```

## Key Source Files

### `src/bin/generate_import_macros.rs`
- **Critical Functions**:
  - `safe_name()`: Sanitizes identifiers (replaces `#` with `_`)
  - Path correction logic for `../../output2/` resolution
  - Callback name generation using safe_name

### Enhanced Syn Parsing (in multiple binaries)
- **Visitor Pattern**: Extracts names from:
  - Macro invocation tokens
  - Attribute tokens  
  - Conditional compilation blocks
  - Standard AST nodes
- **Key Improvement**: Handles `#[cfg(...)]` and macro content

## Build Integration

### `build.rs`
- **Role**: Orchestrates macro generation at compile time
- **Calls**: `generate_import_macros` binary
- **Output**: Regenerates `import_macros.rs` and `src/generated_mkbin.rs`

### Cargo Integration
- **Dependencies**: syn with "full" and "visit" features
- **Build Dependencies**: Configured for macro generation
- **Execution**: `cargo run --bin <binary>` for each tool

## Data Flow Chain

```
1. create_name_index → name_index.json
2. recursive_resolver → recursive_dependencies.json  
3. generate_import_macros → import_macros.rs
4. generate_dependency_macros → dependency_macros.rs
5. build.rs → src/generated_mkbin.rs (INCOMPLETE)
6. test_simple → compilation (FAILS - missing deps)
```

## Current Status

| Component | Status | Coverage |
|-----------|--------|----------|
| Name indexing | ✅ Complete | 141,578 names |
| Dependency resolution | ✅ Complete | 12,511 deps (100%) |
| Import macro generation | ✅ Complete | 12,511 macros |
| Execution macro generation | ✅ Complete | 12,511 calls |
| mkbin generation | ❌ Incomplete | 11 of 12,511 (0.09%) |
| Compilation | ❌ Failing | Missing dependencies |

## Fix Required

**Update mkbin generation** to include all 12,511 dependencies from `recursive_dependencies.json` instead of just the initial rustc terms. This will ensure all discovered dependencies are executed and available during compilation.
