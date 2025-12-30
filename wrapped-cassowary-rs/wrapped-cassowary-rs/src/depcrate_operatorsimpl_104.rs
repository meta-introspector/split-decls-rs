// Generated macro for impl_104 (impl)
macro_rules! Depcrate_operatorsimpl_104 {
() => {
// Module: crate::operators
// Provides: {"impl_104"}
// Dependencies: {}
impl ops :: Sub < Expression > for f64 { type Output = Expression ; fn sub (self , mut e : Expression) -> Expression { e . negate () ; e . constant += self ; e } }
};
}
