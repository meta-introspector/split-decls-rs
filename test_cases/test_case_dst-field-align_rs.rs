// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/example/dst-field-align.rs
// Error: expected square brackets
// Problematic line: line 3

// run-pass
#![allow(dead_code)]
struct Foo<T: ?Sized> {
    a: u16,
    b: T,
}
