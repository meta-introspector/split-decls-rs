// Generated macro for all_ranges (function)
macro_rules! Depcrate_matches_overlapping_armsall_ranges {
() => {
// Module: crate::matches::overlapping_arms
// Provides: {"all_ranges"}
// Dependencies: {}
# [doc = " Gets the ranges for each range pattern arm. Applies `ty` bounds for open ranges."] fn all_ranges < 'tcx > (cx : & LateContext < 'tcx > , arms : & 'tcx [Arm < '_ >] , ty : Ty < 'tcx >) -> Vec < SpannedRange < FullInt > > { arms . iter () . filter_map (| arm | { if let Arm { pat , guard : None , .. } = * arm { if let PatKind :: Range (ref lhs , ref rhs , range_end) = pat . kind { let lhs_const = if let Some (lhs) = lhs { ConstEvalCtxt :: new (cx) . eval_pat_expr (lhs) ? } else { Constant :: new_numeric_min (cx . tcx , ty) ? } ; let rhs_const = if let Some (rhs) = rhs { ConstEvalCtxt :: new (cx) . eval_pat_expr (rhs) ? } else { Constant :: new_numeric_max (cx . tcx , ty) ? } ; let lhs_val = lhs_const . int_value (cx . tcx , ty) ? ; let rhs_val = rhs_const . int_value (cx . tcx , ty) ? ; let rhs_bound = match range_end { RangeEnd :: Included => EndBound :: Included (rhs_val) , RangeEnd :: Excluded => EndBound :: Excluded (rhs_val) , } ; return Some (SpannedRange { span : pat . span , node : (lhs_val , rhs_bound) , }) ; } if let PatKind :: Expr (value) = pat . kind { let value = ConstEvalCtxt :: new (cx) . eval_pat_expr (value) ? . int_value (cx . tcx , cx . typeck_results () . node_type (pat . hir_id)) ? ; return Some (SpannedRange { span : pat . span , node : (value , EndBound :: Included (value)) , }) ; } } None }) . collect () }
};
}
