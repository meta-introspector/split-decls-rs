// Generated macro for impl_93 (impl)
macro_rules! Depcrate_any_calendarimpl_93 {
() => {
// Module: crate::any_calendar
// Provides: {"impl_93"}
// Dependencies: {}
impl IntoAnyCalendar for Japanese { # [inline] fn to_any (self) -> AnyCalendar { AnyCalendar :: Japanese (self) } # [inline] fn kind (& self) -> AnyCalendarKind { AnyCalendarKind :: Japanese } # [inline] fn from_any (any : AnyCalendar) -> Result < Self , AnyCalendar > { if let AnyCalendar :: Japanese (cal) = any { Ok (cal) } else { Err (any) } } # [inline] fn from_any_ref (any : & AnyCalendar) -> Option < & Self > { if let AnyCalendar :: Japanese (cal) = any { Some (cal) } else { None } } # [inline] fn date_to_any (& self , d : & Self :: DateInner) -> AnyDateInner { AnyDateInner :: Japanese (* d) } }
};
}
