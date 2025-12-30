// Generated macro for impl_193 (impl)
macro_rules! Depcrate_unit_displayimpl_193 {
() => {
// Module: crate::unit::display
// Provides: {"impl_193"}
// Dependencies: {}
impl UnitDisplay < '_ > { # [doc = " Display everything, values and the unit."] pub fn all (& mut self) -> & Self { self . display = What :: ValuesAndUnit ; self } # [doc = " Display only values."] pub fn values (& mut self) -> & Self { self . display = What :: Values ; self } # [doc = " Display only units."] pub fn unit (& mut self) -> & Self { self . display = What :: Unit ; self } }
};
}
