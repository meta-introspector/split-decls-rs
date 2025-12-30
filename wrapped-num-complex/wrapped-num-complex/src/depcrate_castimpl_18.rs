// Generated macro for impl_18 (impl)
macro_rules! Depcrate_castimpl_18 {
() => {
// Module: crate::cast
// Provides: {"impl_18"}
// Dependencies: {}
impl < T : NumCast + Num > NumCast for Complex < T > { fn from < U : ToPrimitive > (n : U) -> Option < Self > { Some (Complex { re : T :: from (n) ? , im : T :: zero () , }) } }
};
}
