// Generated macro for IntoAnyCalendar (trait)
macro_rules! Depcrate_any_calendarIntoAnyCalendar {
() => {
// Module: crate::any_calendar
// Provides: {"IntoAnyCalendar"}
// Dependencies: {}
# [doc = " Trait for calendars that may be converted to [`AnyCalendar`]"] pub trait IntoAnyCalendar : Calendar + Sized { # [doc = " Convert this calendar into an [`AnyCalendar`], moving it"] # [doc = ""] # [doc = " You should not need to call this method directly"] fn to_any (self) -> AnyCalendar ; # [doc = " The [`AnyCalendarKind`] enum variant associated with this calendar"] fn kind (& self) -> AnyCalendarKind ; # [doc = " Move an [`AnyCalendar`] into a `Self`, or returning it as an error"] # [doc = " if the types do not match."] # [doc = ""] # [doc = " You should not need to call this method directly"] fn from_any (any : AnyCalendar) -> Result < Self , AnyCalendar > ; # [doc = " Convert an [`AnyCalendar`] reference into a `Self` reference."] # [doc = ""] # [doc = " You should not need to call this method directly"] fn from_any_ref (any : & AnyCalendar) -> Option < & Self > ; # [doc = " Convert a date for this calendar into an `AnyDateInner`"] # [doc = ""] # [doc = " You should not need to call this method directly"] fn date_to_any (& self , d : & Self :: DateInner) -> AnyDateInner ; }
};
}
