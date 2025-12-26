# Recursive Evaluation Demo

This is a **proof-of-concept demonstration** created during the development of the recursive self-execution capability.

## Purpose

This standalone crate demonstrates the concept of split-decls-rs executing its own wrapped main function through the macro system.

## Status

**SUPERSEDED** - The real functionality is now integrated into the main split-decls-rs system:

- `src/all_file_scanner.rs` - Scans all .rs files for functions
- `src/bin/enhanced_wrapper.rs` - Wraps all functions from all files (298 functions from 124 files)
- `src/bin/real_recursive_proof.rs` - Proves actual recursive execution with real wrapped functions
- `enhanced_output/decls/wrapped_main_5.rs` - The actual wrapped main function

## Usage

```bash
cargo run --bin recursive_eval
```

## Note

This was created as a conceptual demonstration. The **real recursive execution** is now achieved through the enhanced wrapper system that processes all functions from all source files, not just lib.rs.
