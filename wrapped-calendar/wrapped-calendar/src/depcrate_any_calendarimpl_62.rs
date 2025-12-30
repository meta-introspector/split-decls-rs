// Generated macro for impl_62 (impl)
macro_rules! Depcrate_any_calendarimpl_62 {
() => {
// Module: crate::any_calendar
// Provides: {"impl_62"}
// Dependencies: {}
impl < C : AsCalendar < Calendar = AnyCalendar > > Date < C > { # [doc = " Convert this `Date<AnyCalendar>` to another `AnyCalendar`, if conversion is needed"] pub fn convert_any < 'a > (& self , calendar : & 'a AnyCalendar) -> Date < Ref < 'a , AnyCalendar > > { if calendar . kind () != self . calendar . as_calendar () . kind () { Date :: new_from_iso (self . to_iso () , Ref (calendar)) } else { Date { inner : self . inner , calendar : Ref (calendar) , } } } }
};
}
