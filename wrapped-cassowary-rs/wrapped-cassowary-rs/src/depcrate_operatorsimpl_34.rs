// Generated macro for impl_34 (impl)
macro_rules! Depcrate_operatorsimpl_34 {
() => {
// Module: crate::operators
// Provides: {"impl_34"}
// Dependencies: {}
impl ops :: Add < Expression > for Variable { type Output = Expression ; fn add (self , mut e : Expression) -> Expression { e . terms . push (Term :: new (self , 1.0)) ; e } }
};
}
