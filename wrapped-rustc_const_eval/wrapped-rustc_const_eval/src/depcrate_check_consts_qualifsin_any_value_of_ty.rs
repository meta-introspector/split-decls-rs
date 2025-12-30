// Generated macro for in_any_value_of_ty (function)
macro_rules! Depcrate_check_consts_qualifsin_any_value_of_ty {
() => {
// Module: crate::check_consts::qualifs
// Provides: {"in_any_value_of_ty"}
// Dependencies: {}
pub fn in_any_value_of_ty < 'tcx > (cx : & ConstCx < '_ , 'tcx > , ty : Ty < 'tcx > , tainted_by_errors : Option < ErrorGuaranteed > ,) -> ConstQualifs { ConstQualifs { has_mut_interior : HasMutInterior :: in_any_value_of_ty (cx , ty) , needs_drop : NeedsDrop :: in_any_value_of_ty (cx , ty) , needs_non_const_drop : NeedsNonConstDrop :: in_any_value_of_ty (cx , ty) , tainted_by_errors , } }
};
}
