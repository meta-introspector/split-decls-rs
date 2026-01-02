// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_resolve/src/imports.rs
// Error: expected square brackets
// Problematic line: line 17

use rustc_middle::span_bug;
use rustc_middle::ty::Visibility;
use rustc_session::lint::BuiltinLintDiag;
use rustc_session::lint::builtin::{
    AMBIGUOUS_GLOB_REEXPORTS, EXPORTED_PRIVATE_DEPENDENCIES, HIDDEN_GLOB_REEXPORTS,
    PUB_USE_OF_PRIVATE_EXTERN_CRATE, REDUNDANT_IMPORTS, UNUSED_IMPORTS,
};
