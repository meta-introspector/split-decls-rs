// Generated macro for impl_36 (impl)
macro_rules! Depcrate_dateimpl_36 {
() => {
// Module: crate::date
// Provides: {"impl_36"}
// Dependencies: {}
impl < A : AsCalendar > fmt :: Debug for Date < A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { let month = self . month () . ordinal ; let day = self . day_of_month () . 0 ; let calendar = self . calendar . as_calendar () . debug_name () ; match self . year () { types :: YearInfo :: Era (EraYear { year , era , .. }) => { write ! (f , "Date({year}-{month}-{day}, {era} era, for calendar {calendar})") } types :: YearInfo :: Cyclic (CyclicYear { year , related_iso }) => { write ! (f , "Date({year}-{month}-{day}, ISO year {related_iso}, for calendar {calendar})") } } } }
};
}
