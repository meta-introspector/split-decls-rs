// Generated macro for check (function)
macro_rules! Depcrate_casts_cast_slice_different_sizescheck {
() => {
// Module: crate::casts::cast_slice_different_sizes
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < 'tcx > , msrv : Msrv) { if is_child_of_cast (cx , expr) { return ; } if let Some (CastChainInfo { left_cast , start_ty , end_ty , }) = expr_cast_chain_tys (cx , expr) && let (Ok (from_layout) , Ok (to_layout)) = (cx . layout_of (start_ty . ty) , cx . layout_of (end_ty . ty)) { let from_size = from_layout . size . bytes () ; let to_size = to_layout . size . bytes () ; if from_size != to_size && from_size != 0 && to_size != 0 && msrv . meets (cx , msrvs :: PTR_SLICE_RAW_PARTS) { span_lint_and_then (cx , CAST_SLICE_DIFFERENT_SIZES , expr . span , format ! ("casting between raw pointers to `[{}]` (element size {from_size}) and `[{}]` (element size {to_size}) does not adjust the count" , start_ty . ty , end_ty . ty ,) , | diag | { let ptr_snippet = source :: snippet (cx , left_cast . span , "..") ; let (mutbl_fn_str , mutbl_ptr_str) = match end_ty . mutbl { Mutability :: Mut => ("_mut" , "mut") , Mutability :: Not => ("" , "const") , } ; let sugg = format ! ("core::ptr::slice_from_raw_parts{mutbl_fn_str}({ptr_snippet} as *{mutbl_ptr_str} {}, ..)" , end_ty . ty) ; diag . span_suggestion (expr . span , format ! ("replace with `ptr::slice_from_raw_parts{mutbl_fn_str}`") , sugg , rustc_errors :: Applicability :: HasPlaceholders ,) ; } ,) ; } } }
};
}
