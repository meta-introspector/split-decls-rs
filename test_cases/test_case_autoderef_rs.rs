// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_typeck/src/autoderef.rs
// Error: expected square brackets
// Problematic line: line 16

use super::method::MethodCallee;
use super::{FnCtxt, PlaceOp};

impl<'a, 'tcx> FnCtxt<'a, 'tcx> {
    pub(crate) fn autoderef(&'a self, span: Span, base_ty: Ty<'tcx>) -> Autoderef<'a, 'tcx> {
        Autoderef::new(self, self.param_env, self.body_id, span, base_ty)
    }
