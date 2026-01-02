// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/mir/visit.rs
// Error: expected square brackets
// Problematic line: line 68

use crate::mir::*;
use crate::ty::CanonicalUserTypeAnnotation;

macro_rules! make_mir_visitor {
    ($visitor_trait_name:ident, $($mutability:ident)?) => {
        pub trait $visitor_trait_name<'tcx> {
            // Override these, and call `self.super_xxx` to revert back to the
