// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/ty/diagnostics.rs
// Error: expected square brackets
// Problematic line: line 7

use std::ops::ControlFlow;

use rustc_data_structures::fx::FxIndexMap;
use rustc_errors::{
    Applicability, Diag, DiagArgValue, IntoDiagArg, into_diag_arg_using_display, listify, pluralize,
};
use rustc_hir::def::{DefKind, Namespace};
