// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_gcc/example/alloc_example.rs
// Error: expected square brackets
// Problematic line: line 13


use alloc_system::System;

#[global_allocator]
static ALLOC: System = System;

#[link(name = "c")]
