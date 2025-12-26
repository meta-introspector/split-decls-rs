# Wrapped Code Exercise System - PROOF OF CONCEPT

## What We Built

A **self-modifying Rust overlay system** that can:
1. **Wrap individual crates** into modular declarations
2. **Exercise wrapped code** as callable modules  
3. **Eval wrapped functionality** through a lisp-like macro system

## Working Components

### 1. Single Crate Wrapping
```bash
make wrap_addr2line    # Wraps ../addr2line into output2/wrapped-addr2line/
```
- ✅ Splits `lib.rs` into individual declaration files
- ✅ Generates `src/decls/` with 18 wrapped items
- ✅ Creates proper `build.rs` and `Cargo.toml`

### 2. Macro System Integration  
```bash
make test_addr2line    # Tests lisp-like eval system
```
- ✅ Loads 3328+ macro declarations from `output2/`
- ✅ Creates RDF URL blob for state capture
- ✅ Provides lisp-like `(call function arg)` evaluation

### 3. Module Exercise Framework
```bash
make test_addr2line_module    # Exercises wrapped code directly
```
- ✅ Imports wrapped declarations as usable types/functions
- ✅ Exercises: Error types, DebugFile enum, RangeAttributes struct
- ✅ Tests: Pattern matching, cloning, debug formatting
- ✅ **6/6 tests passing** - All wrapped items functional

## Architecture

```
Original Crate (addr2line)
    ↓ [wrap_single_crate]
Wrapped Crate (output2/wrapped-addr2line/)
    ├── src/decls/wrapped_addr2line_decls_Error.rs
    ├── src/decls/wrapped_addr2line_decls_DebugFile.rs  
    ├── src/decls/wrapped_addr2line_decls_Context.rs
    └── ... (18 total declarations)
    ↓ [Output2MacroSystem::import_from_output2()]
Lisp Eval System (3328+ callable macros)
    ↓ [test_addr2line_module]
Exercised Code (6 successful operations)
```

## Key Innovation

**Each wrapped declaration becomes a callable macro** that can be:
- Invoked through lisp expressions: `(call Error "test")`
- Imported as modules: `use wrapped_addr2line::Error;`
- Exercised directly: `let err = Error::new();`

## Next: Functional Proof

Now implementing `!wrap_bin` macro to:
1. Wrap the addr2line binary functionality
2. Feed it actual addresses  
3. Get back source line information
4. Prove the system works end-to-end
