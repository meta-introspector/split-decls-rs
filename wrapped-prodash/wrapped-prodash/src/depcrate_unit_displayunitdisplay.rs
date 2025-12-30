// Generated macro for UnitDisplay (struct)
macro_rules! Depcrate_unit_displayUnitDisplay {
() => {
// Module: crate::unit::display
// Provides: {"UnitDisplay"}
// Dependencies: {}
# [doc = " A utility to implement [Display][std::fmt::Display]."] pub struct UnitDisplay < 'a > { pub (crate) current_value : Step , pub (crate) upper_bound : Option < Step > , pub (crate) throughput : Option < Throughput > , pub (crate) parent : & 'a Unit , pub (crate) display : What , }
};
}
