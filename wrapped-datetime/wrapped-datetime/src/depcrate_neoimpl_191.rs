// Generated macro for impl_191 (impl)
macro_rules! Depcrate_neoimpl_191 {
() => {
// Module: crate::neo
// Provides: {"impl_191"}
// Dependencies: {}
impl < C : CldrCalendar , FSet : DateTimeMarkers > FixedCalendarDateTimeFormatter < C , FSet > where FSet :: D : DateInputMarkers , FSet :: T : TimeMarkers , FSet :: Z : ZoneMarkers , { # [doc = " Formats a datetime. Calendars and fields must match at compile time."] pub fn format < I > (& self , input : & I) -> FormattedDateTime < '_ > where I : ? Sized + InFixedCalendar < C > + AllInputMarkers < FSet > , { let input = DateTimeInputUnchecked :: extract_from_neo_input :: < FSet :: D , FSet :: T , FSet :: Z , I > (input) ; FormattedDateTime { pattern : self . selection . select (& input) , input , names : self . names . as_borrowed () , } } }
};
}
