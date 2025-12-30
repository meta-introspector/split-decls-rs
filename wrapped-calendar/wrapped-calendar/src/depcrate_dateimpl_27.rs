// Generated macro for impl_27 (impl)
macro_rules! Depcrate_dateimpl_27 {
() => {
// Module: crate::date
// Provides: {"impl_27"}
// Dependencies: {}
impl < A : AsCalendar < Calendar = C > , C : Calendar < Year = EraYear > > Date < A > { # [doc = " Returns information about the era for calendars using eras."] pub fn era_year (& self) -> EraYear { self . calendar . as_calendar () . year_info (self . inner ()) } }
};
}
