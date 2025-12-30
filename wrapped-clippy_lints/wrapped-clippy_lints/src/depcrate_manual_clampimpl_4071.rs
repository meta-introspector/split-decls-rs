// Generated macro for impl_4071 (impl)
macro_rules! Depcrate_manual_clampimpl_4071 {
() => {
// Module: crate::manual_clamp
// Provides: {"impl_4071"}
// Dependencies: {}
impl < 'tcx > ClampSuggestion < 'tcx > { # [doc = " This function will return true if and only if you can demonstrate at compile time that min"] # [doc = " is less than max."] fn min_less_than_max (& self , cx : & LateContext < 'tcx >) -> bool { let max_type = cx . typeck_results () . expr_ty (self . params . max) ; let min_type = cx . typeck_results () . expr_ty (self . params . min) ; if max_type != min_type { return false ; } let ecx = ConstEvalCtxt :: new (cx) ; if let Some (max) = ecx . eval (self . params . max) && let Some (min) = ecx . eval (self . params . min) && let Some (ord) = Constant :: partial_cmp (cx . tcx , max_type , & min , & max) { ord != Ordering :: Greater } else { false } } }
};
}
