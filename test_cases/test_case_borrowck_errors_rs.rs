// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_borrowck/src/borrowck_errors.rs
// Error: expected square brackets
// Problematic line: line 11

use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::Span;

impl<'infcx, 'tcx> crate::MirBorrowckCtxt<'_, 'infcx, 'tcx> {
    pub(crate) fn dcx(&self) -> DiagCtxtHandle<'infcx> {
        self.infcx.dcx()
    }
