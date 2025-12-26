# Split-Decls-RS Documentation Series

## Part 1: System Overview & Architecture

### What is Split-Decls-RS?

Split-decls-rs is a **self-modifying Rust overlay system** that transforms any Rust codebase into a modular, addressable, and executable declaration space. It implements a revolutionary approach to code analysis, wrapping, and runtime manipulation.

### Core Innovation

The system takes any Rust crate and:
1. **Splits** it into individual declaration files
2. **Wraps** each declaration as a callable module
3. **Maps** every declaration to a memory address
4. **Enables** lisp-like evaluation of wrapped code

### Architecture Overview

```
Original Crate (e.g., addr2line)
    ↓ [wrap_single_crate]
Wrapped Declarations (output2/wrapped-addr2line/)
    ├── src/decls/wrapped_addr2line_decls_Error.rs
    ├── src/decls/wrapped_addr2line_decls_DebugFile.rs  
    ├── src/decls/wrapped_addr2line_decls_Context.rs
    └── ... (18 total declarations)
    ↓ [Output2MacroSystem::import_from_output2()]
Macro System (3,328+ callable declarations)
    ↓ [decl2addr! & alldecls!]
Address Space (complete memory mapping)
    ↓ [!wrap_bin & eval system]
Executable Runtime (lisp-like evaluation)
```

### Key Components

1. **Wrapper System** - Transforms crates into modular declarations
2. **Macro System** - Imports declarations as callable macros
3. **Address Mapping** - Maps all declarations to memory addresses
4. **Evaluation Engine** - Executes wrapped code through lisp-like expressions
5. **RDF State System** - Captures execution state and metadata

### Proven Capabilities

- ✅ **3,328 declarations** successfully wrapped and mapped
- ✅ **Complete address coverage** - every declaration has an address
- ✅ **Real symbol resolution** - actual memory addresses where available
- ✅ **Type system support** - structs, enums, functions, traits all supported
- ✅ **End-to-end execution** - from source code to runtime evaluation

### Use Cases

- **Code Analysis** - Complete codebase introspection and mapping
- **Debugging Tools** - Address-to-source resolution for any declaration
- **Runtime Modification** - Dynamic code transformation and patching
- **Ecosystem Transformation** - Systematic Rust codebase modernization
- **Research Platform** - Experimental code manipulation and analysis

### Next Steps

This documentation series covers:
- Part 2: Installation & Quick Start
- Part 3: Wrapping System Deep Dive
- Part 4: Address Mapping & Memory Layout
- Part 5: Macro System & Evaluation Engine
- Part 6: Advanced Features & Integration
- Part 7: Extending & Contributing

---
*Split-decls-rs represents a new paradigm in Rust tooling - making every piece of code addressable, callable, and transformable.*
