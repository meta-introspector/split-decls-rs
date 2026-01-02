// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/mir/coverage.rs
// Error: expected square brackets
// Problematic line: line 10

use rustc_macros::{HashStable, TyDecodable, TyEncodable};
use rustc_span::Span;

rustc_index::newtype_index! {
    /// Used by [`CoverageKind::BlockMarker`] to mark blocks during THIR-to-MIR
    /// lowering, so that those blocks can be identified later.
    #[derive(HashStable)]
