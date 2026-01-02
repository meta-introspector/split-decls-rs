// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/ty/closure.rs
// Error: unexpected token, expected `;`
// Error type: unexpected_token
// Sample #3 of 3
// Problematic line: line 12

use rustc_span::{Ident, Span, Symbol};

use super::TyCtxt;
use crate::hir::place::{
    Place as HirPlace, PlaceBase as HirPlaceBase, ProjectionKind as HirProjectionKind,
};
use crate::query::Providers;
