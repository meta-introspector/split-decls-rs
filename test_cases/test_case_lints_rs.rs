// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_lint/src/lints.rs
// Error: expected square brackets
// Problematic line: line 7

use std::num::NonZero;

use rustc_errors::codes::*;
use rustc_errors::{
    Applicability, Diag, DiagArgValue, DiagMessage, DiagStyledString, ElidedLifetimeInPathSubdiag,
    EmissionGuarantee, LintDiagnostic, MultiSpan, Subdiagnostic, SuggestionStyle,
};
