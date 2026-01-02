// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_llvm/src/declare.rs
// Error: expected square brackets
// Problematic line: line 34

use crate::value::Value;
use crate::{attributes, llvm};

/// Declare a function with a SimpleCx.
///
/// If there’s a value with the same name already declared, the function will
/// update the declaration and return existing Value instead.
