// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_errors/src/translation.rs
// Error: expected an expression
// Error type: expected_expression
// Sample #2 of 3
// Problematic line: line 13

use crate::snippet::Style;
use crate::{DiagArg, DiagMessage, FluentBundle};

/// Convert diagnostic arguments (a rustc internal type that exists to implement
/// `Encodable`/`Decodable`) into `FluentArgs` which is necessary to perform translation.
///
/// Typically performed once for each diagnostic at the start of `emit_diagnostic` and then
