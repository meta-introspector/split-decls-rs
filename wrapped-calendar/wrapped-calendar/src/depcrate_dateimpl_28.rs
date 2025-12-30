// Generated macro for impl_28 (impl)
macro_rules! Depcrate_dateimpl_28 {
() => {
// Module: crate::date
// Provides: {"impl_28"}
// Dependencies: {}
impl < A : AsCalendar < Calendar = C > , C : Calendar < Year = CyclicYear > > Date < A > { # [doc = " Returns information about the year cycle, for cyclic calendars."] pub fn cyclic_year (& self) -> CyclicYear { self . calendar . as_calendar () . year_info (self . inner ()) } }
};
}
