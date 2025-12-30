// Generated macro for check (function)
macro_rules! Depcrate_casts_cast_slice_from_raw_partscheck {
() => {
// Module: crate::casts::cast_slice_from_raw_parts
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , cast_expr : & Expr < '_ > , cast_to : Ty < '_ > , msrv : Msrv) { if let ty :: RawPtr (ptrty , _) = cast_to . kind () && let ty :: Slice (_) = ptrty . kind () && let ExprKind :: Call (fun , [ptr_arg , len_arg]) = cast_expr . peel_blocks () . kind && let ExprKind :: Path (ref qpath) = fun . kind && let Some (fun_def_id) = cx . qpath_res (qpath , fun . hir_id) . opt_def_id () && let Some (rpk) = raw_parts_kind (cx , fun_def_id) && let ctxt = expr . span . ctxt () && cast_expr . span . ctxt () == ctxt && msrv . meets (cx , msrvs :: PTR_SLICE_RAW_PARTS) { let func = match rpk { RawPartsKind :: Immutable => "from_raw_parts" , RawPartsKind :: Mutable => "from_raw_parts_mut" , } ; let span = expr . span ; let mut applicability = Applicability :: MachineApplicable ; let ptr = snippet_with_context (cx , ptr_arg . span , ctxt , "ptr" , & mut applicability) . 0 ; let len = snippet_with_context (cx , len_arg . span , ctxt , "len" , & mut applicability) . 0 ; let krate = if is_no_std_crate (cx) { "core" } else { "std" } ; span_lint_and_sugg (cx , CAST_SLICE_FROM_RAW_PARTS , span , format ! ("casting the result of `{func}` to {cast_to}") , "replace with" , format ! ("{krate}::ptr::slice_{func}({ptr}, {len})") , applicability ,) ; } }
};
}
