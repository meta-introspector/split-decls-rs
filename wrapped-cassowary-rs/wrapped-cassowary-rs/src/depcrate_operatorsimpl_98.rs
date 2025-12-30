// Generated macro for impl_98 (impl)
macro_rules! Depcrate_operatorsimpl_98 {
() => {
// Module: crate::operators
// Provides: {"impl_98"}
// Dependencies: {}
impl ops :: AddAssign < Expression > for Expression { fn add_assign (& mut self , mut e : Expression) { self . terms . append (& mut e . terms) ; self . constant += e . constant ; } }
};
}
