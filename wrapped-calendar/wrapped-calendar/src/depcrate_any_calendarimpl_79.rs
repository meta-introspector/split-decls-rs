// Generated macro for impl_79 (impl)
macro_rules! Depcrate_any_calendarimpl_79 {
() => {
// Module: crate::any_calendar
// Provides: {"impl_79"}
// Dependencies: {}
impl IntoAnyCalendar for Gregorian { # [inline] fn to_any (self) -> AnyCalendar { AnyCalendar :: Gregorian (Gregorian) } # [inline] fn kind (& self) -> AnyCalendarKind { AnyCalendarKind :: Gregorian } # [inline] fn from_any (any : AnyCalendar) -> Result < Self , AnyCalendar > { if let AnyCalendar :: Gregorian (cal) = any { Ok (cal) } else { Err (any) } } # [inline] fn from_any_ref (any : & AnyCalendar) -> Option < & Self > { if let AnyCalendar :: Gregorian (cal) = any { Some (cal) } else { None } } # [inline] fn date_to_any (& self , d : & Self :: DateInner) -> AnyDateInner { AnyDateInner :: Gregorian (* d) } }
};
}
