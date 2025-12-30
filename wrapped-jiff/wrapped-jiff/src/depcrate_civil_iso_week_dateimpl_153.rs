// Generated macro for impl_153 (impl)
macro_rules! Depcrate_civil_iso_week_dateimpl_153 {
() => {
// Module: crate::civil::iso_week_date
// Provides: {"impl_153"}
// Dependencies: {}
impl PartialEq for ISOWeekDate { # [inline] fn eq (& self , other : & ISOWeekDate) -> bool { self . weekday == other . weekday && self . week . get () == other . week . get () && self . year . get () == other . year . get () } }
};
}
