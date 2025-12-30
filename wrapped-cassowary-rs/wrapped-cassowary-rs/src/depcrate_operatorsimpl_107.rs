// Generated macro for impl_107 (impl)
macro_rules! Depcrate_operatorsimpl_107 {
() => {
// Module: crate::operators
// Provides: {"impl_107"}
// Dependencies: {}
impl ops :: SubAssign < Expression > for Expression { fn sub_assign (& mut self , mut e : Expression) { e . negate () ; self . terms . append (& mut e . terms) ; self . constant += e . constant ; } }
};
}
