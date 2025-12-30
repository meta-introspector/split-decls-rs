// Generated macro for try_convert_match (function)
macro_rules! Depcrate_option_if_let_elsetry_convert_match {
() => {
// Module: crate::option_if_let_else
// Provides: {"try_convert_match"}
// Dependencies: {}
fn try_convert_match < 'tcx > (cx : & LateContext < 'tcx > , arms : & [Arm < 'tcx >] ,) -> Option < (& 'tcx Pat < 'tcx > , & 'tcx Expr < 'tcx > , & 'tcx Expr < 'tcx >) > { if let [first_arm , second_arm] = arms && first_arm . guard . is_none () && second_arm . guard . is_none () { return if is_none_or_err_arm (cx , second_arm) { Some ((first_arm . pat , first_arm . body , second_arm . body)) } else if is_none_or_err_arm (cx , first_arm) { Some ((second_arm . pat , second_arm . body , first_arm . body)) } else { None } ; } None }
};
}
