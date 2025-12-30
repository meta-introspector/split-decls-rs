// Generated macro for impl_43 (impl)
macro_rules! Depcrate_ulps_eqimpl_43 {
() => {
// Module: crate::ulps_eq
// Provides: {"impl_43"}
// Dependencies: {}
impl ApproxEqUlps for f64 { type Flt = f64 ; fn approx_eq_ulps (& self , other : & f64 , ulps : i64) -> bool { if * self == * other { return true ; } if self . is_sign_positive () != other . is_sign_positive () { return false ; } let diff : i64 = self . ulps (other) ; diff >= - ulps && diff <= ulps } }
};
}
