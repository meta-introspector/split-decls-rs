// Generated macro for impl_78 (impl)
macro_rules! Depcrate_operatorsimpl_78 {
() => {
// Module: crate::operators
// Provides: {"impl_78"}
// Dependencies: {}
impl ops :: Sub < Expression > for Term { type Output = Expression ; fn sub (self , mut e : Expression) -> Expression { e . negate () ; e . terms . push (self) ; e } }
};
}
