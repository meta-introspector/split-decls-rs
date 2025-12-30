// Generated macro for impl_45 (impl)
macro_rules! Depcrate_operatorsimpl_45 {
() => {
// Module: crate::operators
// Provides: {"impl_45"}
// Dependencies: {}
impl ops :: Sub < Expression > for Variable { type Output = Expression ; fn sub (self , mut e : Expression) -> Expression { e . negate () ; e . terms . push (Term :: new (self , 1.0)) ; e } }
};
}
