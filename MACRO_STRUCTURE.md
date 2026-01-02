# Split Declarations Genesis: Macro-Wrapped Module Structure

## Overview

The build.rs system wraps every Rust construct (functions, modules, structs, etc.) in introspective macros that provide metadata and dependency tracking. This creates a structured, analyzable representation of the rustc codebase.

## Macro Wrapper Architecture

### Core Wrapper Macros

#### `mkfn!` - Function Wrapper
Wraps functions with introspection and execution tracking:

```rust
// Original function:
pub fn main() {
    println!("Hello rustc");
}

// Wrapped version:
mkfn!{
    main_introspect!();
    pub fn main() {
        println!("Hello rustc");
    }
}

// Expands to:
pub fn main() {
    main_introspect!();
    emit_message!("🚀 MARKER: pub_non_generic - main");
    let result = (|| {
        println!("Hello rustc");
    })();
    emit_message!("🎯 MARKER: pub_non_generic - main");
    result
}
```

#### `mkmod!` - Module Wrapper
Wraps modules with metadata extraction:

```rust
// Original:
mod session {
    pub fn create() {}
}

// Wrapped:
mkmod!{session, {
    getname!(session);
    getsrc!(session);
    getpath!(session);
    get_deps!(session);
    get_crates!(session);
    mkinclude!(session);
    
    mkfn!{
        create_introspect!();
        pub fn create() {}
    }
}}
```

#### `mkuse!` - Use Statement Wrapper
Tracks import dependencies:

```rust
// Original:
use rustc_middle::ty::TyCtxt;

// Wrapped:
mkuse!{use rustc_middle::ty::TyCtxt;}
```

### Metadata Extraction Macros

Each module gets these introspection capabilities:

```rust
getname!(module_name);     // → pub fn get_module_name() -> &'static str
getsrc!(module_name);      // → pub fn get_source_info() -> &'static str  
getpath!(module_name);     // → pub fn get_module_path() -> &'static str
get_deps!(module_name);    // → pub fn get_dependencies() -> &'static [&'static str]
get_crates!(module_name);  // → pub fn get_required_crates() -> &'static [&'static str]
```

## Generated File Structure

### Processed Files
Each rustc source file becomes a macro-wrapped version:

```
Original: submodules/rust/compiler/rustc_driver_impl/src/lib.rs
Wrapped:  processed_submodules_rust_compiler_rustc_driver_impl_src_lib.rs
```

### Example Wrapped File Structure

```rust
// processed_submodules_rust_compiler_rustc_driver_impl_src_lib.rs

// Auto-generated introspection macros
macro_rules! install_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function install in module {}", module_path!());
    };
}

macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

// Wrapped module structure
mkmod!{rustc_driver_impl, {
    getname!(rustc_driver_impl);
    getsrc!(rustc_driver_impl);
    getpath!(rustc_driver_impl);
    get_deps!(rustc_driver_impl);
    get_crates!(rustc_driver_impl);
    
    // Wrapped use statements
    mkuse!{use std::env;}
    mkuse!{use rustc_session::EarlyDiagCtxt;}
    
    // Wrapped functions
    mkfn!{
        install_introspect!();
        pub fn install() {
            // original function body
        }
    }
    
    mkfn!{
        main_introspect!();
        pub fn main() -> ! {
            // original main function body
        }
    }
}}
```

## Dependency Resolution Integration

### Symbol Map Generation
The macro structure enables precise dependency extraction:

1. **Function-level tracking**: Each `mkfn!` wrapper records function calls
2. **Module-level metadata**: `get_deps!()` provides dependency lists
3. **Crate-level requirements**: `get_crates!()` lists external crate dependencies

### Unified Driver Integration
The unified driver leverages this structure:

```rust
// Load processed file
let processed_content = fs::read_to_string("processed_submodules_rust_compiler_rustc_driver_impl_src_lib.rs")?;

// Extract dependencies using macro introspection
let dependencies = extract_dependencies_from_wrapped_code(&processed_content);

// Generate complete dependency tree
let complete_code = generate_with_all_dependencies(dependencies);
```

## Compilation Strategy

### Phase 1: Macro Expansion
```rust
// Include all processed files with macro expansion
include!("processed_submodules_rust_compiler_rustc_driver_impl_src_lib.rs");
```

### Phase 2: Dependency Resolution
```rust
// Use introspection to build dependency graph
let deps = rustc_driver_impl::get_dependencies();
let crates = rustc_driver_impl::get_required_crates();
```

### Phase 3: Code Generation
```rust
// Generate complete code with all resolved dependencies
let complete_rustc = generate_complete_code_with_deps(target_symbol, deps, crates);
```

## Key Benefits

### 1. **Complete Introspection**
Every function, module, and dependency is trackable at compile time.

### 2. **Precise Dependency Resolution** 
No guesswork - exact dependency relationships are captured in the macro structure.

### 3. **Incremental Compilation**
Individual wrapped modules can be compiled independently with their exact dependencies.

### 4. **Debugging Capabilities**
Execution flow is traceable through the `emit_message!` calls in each wrapper.

## Current Implementation Status

### ✅ Completed
- Macro wrapper system (`mkfn!`, `mkmod!`, `mkuse!`, etc.)
- Introspection macro generation
- Processed file generation for test cases
- Symbol map integration

### 🔄 In Progress  
- Complete rustc codebase processing (3102 files)
- Dependency resolution refinement
- Autofix system for missing symbols

### 🎯 Next Steps
1. **Guide autofix system** to use macro structure for better symbol resolution
2. **Leverage introspection** to build accurate dependency graphs
3. **Generate complete rustc main** using the macro-wrapped modules

## Usage Example

```rust
// Target: rustc_driver_impl::lib::main
let mut driver = UnifiedDriver::new()?;

// Use macro structure for precise resolution
driver.resolve_with_macro_introspection("rustc_driver_impl::lib::main")?;

// Generate complete code using wrapped modules
let complete_code = driver.generate_from_wrapped_modules()?;

// Compile with all dependencies resolved
fs::write("src/current.rs", complete_code)?;
```

This macro-wrapped structure provides the foundation for precise, automated rustc compilation with complete dependency resolution.
