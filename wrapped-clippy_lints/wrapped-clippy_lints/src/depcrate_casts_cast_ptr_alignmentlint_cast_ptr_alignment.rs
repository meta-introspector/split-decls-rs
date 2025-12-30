// Generated macro for lint_cast_ptr_alignment (function)
macro_rules! Depcrate_casts_cast_ptr_alignmentlint_cast_ptr_alignment {
() => {
// Module: crate::casts::cast_ptr_alignment
// Provides: {"lint_cast_ptr_alignment"}
// Dependencies: {}
fn lint_cast_ptr_alignment < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < '_ > , cast_from : Ty < 'tcx > , cast_to : Ty < 'tcx >) { if let ty :: RawPtr (from_ptr_ty , _) = * cast_from . kind () && let ty :: RawPtr (to_ptr_ty , _) = * cast_to . kind () && let Ok (from_layout) = cx . layout_of (from_ptr_ty) && let Ok (to_layout) = cx . layout_of (to_ptr_ty) && from_layout . align . abi < to_layout . align . abi && ! is_c_void (cx , from_ptr_ty) && ! from_layout . is_zst () && ! is_used_as_unaligned (cx , expr) { span_lint (cx , CAST_PTR_ALIGNMENT , expr . span , format ! ("casting from `{cast_from}` to a more-strictly-aligned pointer (`{cast_to}`) ({} < {} bytes)" , from_layout . align . bytes () , to_layout . align . bytes () ,) ,) ; } }
};
}
