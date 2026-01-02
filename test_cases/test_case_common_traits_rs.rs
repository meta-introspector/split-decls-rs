// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_ty_utils/src/common_traits.rs
// Error: expected square brackets
// Problematic line: line 10

use rustc_span::DUMMY_SP;
use rustc_trait_selection::traits;

fn is_copy_raw<'tcx>(tcx: TyCtxt<'tcx>, query: ty::PseudoCanonicalInput<'tcx, Ty<'tcx>>) -> bool {
    is_item_raw(tcx, query, LangItem::Copy)
}

