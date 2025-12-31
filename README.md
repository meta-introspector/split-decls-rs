# split-decls-genesis

Self-creating Rust system through macro emergence and dependency analysis.

## Rustc Dependency Analysis Pipeline

Extract and analyze rustc compiler dependencies to generate optimal build order.

### Core Programs

#### 1. `export_symbol_map`
Generates complete symbol map from all rustc files and submodules.

**Output**: `symbol_map.json` (140,199 symbols from 3,545 files)

```bash
cargo run --bin export_symbol_map
```

**Features**:
- Scans all rustc crates and their submodules
- Extracts symbols from 75+ rustc crates
- Processes 1000+ submodule files (diagnostics, etc.)
- Creates comprehensive dependency database

#### 2. `complete_rustc_analysis`
Generates complete recursive function call trace from rustc entry points.

**Input**: `symbol_map.json`  
**Output**: `rustc_complete_analysis.txt` (indented function call tree)

```bash
cargo run --bin complete_rustc_analysis
```

**Features**:
- Traces `rustc::main::main` and `rustc_driver_impl::lib::main`
- Recursive dependency resolution with cycle detection
- Call frequency statistics
- Hierarchical output with indentation levels

#### 3. `fix_lattice_petgraph`
Converts function call tree into dependency graph and compilation lattice.

**Input**: `rustc_complete_analysis.txt`  
**Output**: `rustc_fixed_lattice.txt` (dependency levels for compilation)

```bash
cargo run --bin fix_lattice_petgraph
```

**Features**:
- Petgraph-based topological sorting
- Cycle detection and reporting
- Dependency level calculation (0=leaves, max=main)
- Compilation order optimization

#### 4. Build.rs System
Creates include! statements for all rustc modules in topological dependency order.

**Input**: `symbol_map.json` (direct JSON parsing)  
**Output**: `src/rustc_includes.rs` (module declarations with #[path])

**Features**:
- Processes all 3,545 rustc files including submodules
- Generates proper module structure with path attributes
- Handles crate:: path rewriting for module conflicts
- Enables compile-time inclusion of entire rustc compiler

### Complete Workflow

```bash
# 1. Generate complete symbol map (all files + submodules)
cargo run --bin export_symbol_map

# 2. Generate function call analysis
cargo run --bin complete_rustc_analysis

# 3. Create dependency lattice  
cargo run --bin fix_lattice_petgraph

# 4. Build system automatically uses symbol_map.json
cargo build
```

### Key Achievement

**✅ Successfully includes all rustc modules in dependency order via include! statements**

The build.rs system now processes the complete rustc compiler (140,199 symbols from 3,545 files) and generates proper Rust module declarations, enabling compile-time access to the entire rustc codebase as first-class modules.

### Output Files

- `symbol_map.json` - Complete symbol database (140,199 symbols)
- `rustc_complete_analysis.txt` - Function call hierarchy
- `rustc_fixed_lattice.txt` - Dependency levels (leaves → main)
- `src/rustc_includes.rs` - Generated module declarations

### Dependencies

- `petgraph` - Graph algorithms for dependency analysis
- `serde_json` - Symbol map parsing
- `syn` - Rust code analysis
- `walkdir` - File system traversal

This pipeline enables optimal rustc compilation by ensuring dependencies are resolved before dependents, supporting the broader cargo2nix quasi-meta computationally self-aware system goals.
