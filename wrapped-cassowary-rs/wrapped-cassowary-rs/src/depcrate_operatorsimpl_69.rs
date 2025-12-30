// Generated macro for impl_69 (impl)
macro_rules! Depcrate_operatorsimpl_69 {
() => {
// Module: crate::operators
// Provides: {"impl_69"}
// Dependencies: {}
impl ops :: Add < Expression > for Term { type Output = Expression ; fn add (self , mut e : Expression) -> Expression { e . terms . push (self) ; e } }
};
}
