// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_macros/src/diagnostics/subdiagnostic.rs
// Error: expected square brackets
// Problematic line: line 10

use synstructure::{BindingInfo, Structure, VariantInfo};

use super::utils::SubdiagnosticVariant;
use crate::diagnostics::error::{
    DiagnosticDeriveError, invalid_attr, span_err, throw_invalid_attr, throw_span_err,
};
use crate::diagnostics::utils::{
