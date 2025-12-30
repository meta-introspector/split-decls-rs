// Generated macro for wilkinson_shift (function)
macro_rules! Depcrate_linalg_symmetric_eigenwilkinson_shift {
() => {
// Module: crate::linalg::symmetric_eigen
// Provides: {"wilkinson_shift"}
// Dependencies: {}
# [doc = " Computes the wilkinson shift, i.e., the 2x2 symmetric matrix eigenvalue to its tailing"] # [doc = " component `tnn`."] # [doc = ""] # [doc = " The inputs are interpreted as the 2x2 matrix:"] # [doc = "     tmm  tmn"] # [doc = "     tmn  tnn"] pub fn wilkinson_shift < T : ComplexField > (tmm : T , tnn : T , tmn : T) -> T { let sq_tmn = tmn . clone () * tmn ; if ! sq_tmn . is_zero () { let d = (tmm - tnn . clone ()) * crate :: convert (0.5) ; tnn - sq_tmn . clone () / (d . clone () + d . clone () . signum () * (d . clone () * d + sq_tmn) . sqrt ()) } else { tnn } }
};
}
