// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/example/gen_block_iterate.rs
// Error: expected square brackets
// Problematic line: line 8

// run-pass
#![feature(gen_blocks)]

fn foo() -> impl Iterator<Item = u32> {
    gen {
        yield 42;
        for x in 3..6 {
