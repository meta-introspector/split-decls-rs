// Generated macro for impl_85 (impl)
macro_rules! Depcrate_operatorsimpl_85 {
() => {
// Module: crate::operators
// Provides: {"impl_85"}
// Dependencies: {}
impl ops :: Mul < Expression > for f64 { type Output = Expression ; fn mul (self , mut e : Expression) -> Expression { e . constant *= self ; for t in & mut e . terms { * t = * t * self ; } e } }
};
}
