// Generated macro for impl_39 (impl)
macro_rules! Depcrate_ulps_eqimpl_39 {
() => {
// Module: crate::ulps_eq
// Provides: {"impl_39"}
// Dependencies: {}
impl ApproxEqUlps for f32 { type Flt = f32 ; fn approx_eq_ulps (& self , other : & f32 , ulps : i32) -> bool { if * self == * other { return true ; } if self . is_sign_positive () != other . is_sign_positive () { return false ; } let diff : i32 = self . ulps (other) ; diff >= - ulps && diff <= ulps } }
};
}
