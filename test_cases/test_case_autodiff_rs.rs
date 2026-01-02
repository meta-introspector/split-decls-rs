// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_builtin_macros/src/autodiff.rs
// Error: expected square brackets
// Problematic line: line 6

//! configs (autodiff enabled or disabled), so we have to add cfg's to each import.
//! FIXME(ZuseZ4): Remove this once we have a smarter linter.

mod llvm_enzyme {
    use std::str::FromStr;
    use std::string::String;

