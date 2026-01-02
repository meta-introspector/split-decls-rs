# Split Declarations Genesis - Syn Usage Audit

## Overview
This project extensively uses the `syn` crate for Rust AST parsing and code generation. We have built a comprehensive ecosystem of tools that process rustc source code and generate wrapper modules.

## Core Architecture

### 1. Main Build System (`build.rs`)
**Purpose**: Primary AST processing engine that wraps rustc source files with macro-based introspection
**Syn Usage**:
- `syn::parse_file()` - Parse entire Rust source files into AST
- `syn::Item` - Process different types of AST items (modules, functions, structs, etc.)
- Pattern matching on `Item::Mod`, `Item::Use`, `Item::Fn`, `Item::Struct`, `Item::Enum`, `Item::Trait`, `Item::Impl`

**Key Features**:
- Wraps each AST item with introspection macros (`mkmod!`, `mkfn!`, `mkstruct!`, etc.)
- Processes rustc source files and generates wrapped versions
- Fixes file paths, environment variables, and doc comments
- Generates processed files with macro wrappers for dependency tracking

### 2. Symbol Map Generation (`src/bin/export_symbol_map.rs`)
**Purpose**: Extracts symbol dependencies from processed files
**Syn Usage**: Likely uses syn for parsing and extracting symbol information
**Output**: `symbol_map.json.gz` - Complete dependency database with 140,199 symbols

### 3. Syn-Based Module Generators

#### A. `src/bin/syn_generator.rs`
**Purpose**: Advanced syn-based module generator with source tracking
**Syn Usage**:
- `syn::parse_str<ItemStruct>()` - Parse struct definitions from strings
- `syn::Ident::new()` - Create identifiers programmatically
- `quote!` macro for code generation
- `proc_macro2::TokenStream` for token manipulation

**Features**:
- Extracts `super::` imports from processed files
- Groups imports by module and type
- Generates comprehensive module structure with source documentation
- Creates stub types with exact source file and line number tracking
- Uses syn to validate generated struct definitions

#### B. `src/bin/clean_macro_extractor.rs`
**Purpose**: Simpler module generator focused on import extraction
**Features**:
- Collects `super::` imports from processed files
- Creates mirror directory structure
- Generates basic stub types without syn validation
- More lightweight than syn_generator

#### C. `src/bin/macro_extractor.rs`
**Purpose**: Macro-based skeleton generator
**Features**:
- Extracts macro definitions from processed files
- Generates macro skeletons for file inclusion
- Creates deterministic macro names from content hashes

### 4. Specialized Extractors

#### A. `src/bin/stub_generator.rs`
**Purpose**: Generates stub implementations for missing types
**Likely Syn Usage**: AST parsing for stub generation

#### B. `src/bin/interface_extractor.rs`
**Purpose**: Extracts interface definitions from rustc code
**Likely Syn Usage**: AST analysis for interface extraction

#### C. `src/bin/batch_genmocks.rs`
**Purpose**: Batch generation of mock implementations
**Likely Syn Usage**: AST processing for mock generation

### 5. Dependency Resolution System

#### A. `src/bin/unified_driver.rs`
**Purpose**: Declarative dependency resolution engine
**Features**:
- Loads symbol map for recursive dependency resolution
- Resolves ALL dependencies for a target symbol
- Generates complete code trees with proper module order
- Auto-fix system for missing symbols

#### B. `src/symbol_resolver.rs`
**Syn Usage**: Heavy syn usage (98 matches)
- `syn::File` - Parse complete files
- `syn::visit::Visit` - AST visitor pattern
- `syn::Item::Use`, `syn::Item::Fn` - Process specific item types
- Comprehensive symbol extraction and dependency tracking

### 6. Runbuild Crate (`./runbuild/`)
**Purpose**: Standalone build.rs execution for debugging
**Structure**:
- Separate crate with syn dependencies
- Includes the main build.rs for isolated testing
- Enables debugging of build process without full compilation

## Key Syn Features Used

### AST Parsing
- `syn::parse_file()` - Parse entire Rust files
- `syn::parse_str()` - Parse code from strings
- `syn::Item` and variants - Process different AST node types

### Code Generation
- `quote!` macro - Generate Rust code
- `proc_macro2::TokenStream` - Token manipulation
- `syn::Ident::new()` - Create identifiers

### AST Traversal
- `syn::visit::Visit` - Visitor pattern for AST traversal
- Pattern matching on AST nodes

### Validation
- Syn's parsing validates generated code structure
- Error handling for malformed AST

## Generated Artifacts

### 1. Processed Files
- `processed_submodules_rust_compiler_*.rs` - Wrapped rustc source files
- Each file contains original code wrapped with introspection macros

### 2. Module Definitions
- `src/wrap_types.rs` - Generated module structure with stub types
- Contains comprehensive type definitions extracted from rustc

### 3. Symbol Database
- `symbol_map.json.gz` - Complete dependency relationships
- 140,199 symbols with full dependency metadata

### 4. Proof Files
- `proofs/` directory - AST trace documentation for each processed file

## Integration Points

### 1. With Rustc Source
- Processes 3,545 rustc source files
- Extracts 140,199 symbols with dependencies
- Maintains original directory structure in `submodules/`

### 2. With Compilation System
- `unified_rustc_wrapped.rs` - Main binary that uses generated modules
- `src/rustc_complete.rs` - Module that includes processed types
- Custom include macros for processed file integration

### 3. With Dependency Resolution
- Symbol map drives automatic dependency resolution
- Progressive compilation testing system
- Auto-fix cache for missing symbols

## Current Status

### Achievements
- ✅ Complete rustc source processing (3,545 files)
- ✅ Comprehensive symbol extraction (140,199 symbols)
- ✅ Working dependency resolution system
- ✅ Progressive compilation testing
- ✅ Macro-based wrapper system
- ✅ Real rustc type integration (replacing stubs)

### Recent Breakthrough
- Successfully replaced stub types with real processed rustc types
- Reduced compilation errors from 90+ to just a few
- Implemented custom include macros for processed files
- Created resolver macro system for automatic type resolution

## Development Workflow

### 1. Source Processing
```bash
cargo run --bin runbuild  # Process rustc source files
```

### 2. Module Generation
```bash
cargo run --bin syn_generator  # Generate wrap_types.rs with syn
```

### 3. Dependency Resolution
```bash
cargo run --bin unified_driver "target_symbol"  # Resolve dependencies
```

### 4. Compilation Testing
```bash
cargo check --bin unified_rustc_wrapped  # Test compilation
```

## Future Enhancements

### 1. Enhanced Syn Usage
- More sophisticated AST analysis
- Better error recovery in parsing
- Advanced code generation patterns

### 2. Improved Module Generation
- Dynamic stub generation based on usage patterns
- Better type inference from AST analysis
- Automated conflict resolution

### 3. Integration Improvements
- Better integration with cargo2nix ecosystem
- Enhanced parallel processing
- Improved caching mechanisms

## Conclusion

This project represents one of the most comprehensive uses of the `syn` crate for large-scale Rust code processing. We've built a complete ecosystem that:

1. **Processes the entire rustc codebase** using syn for AST parsing
2. **Generates comprehensive module structures** with syn-validated code
3. **Provides declarative dependency resolution** based on AST analysis
4. **Enables incremental compilation** of rustc components
5. **Maintains full traceability** from original source to generated code

The syn-based architecture enables us to treat the rustc codebase as data, allowing for sophisticated analysis, transformation, and code generation that would be impossible with simple text processing.
