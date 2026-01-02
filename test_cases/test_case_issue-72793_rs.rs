// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/example/issue-72793.rs
// Error: expected square brackets
// Problematic line: line 5


#![feature(type_alias_impl_trait)]

pub trait T {
    type Item;
}

