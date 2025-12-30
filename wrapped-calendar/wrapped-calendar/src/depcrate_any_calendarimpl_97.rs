// Generated macro for impl_97 (impl)
macro_rules! Depcrate_any_calendarimpl_97 {
() => {
// Module: crate::any_calendar
// Provides: {"impl_97"}
// Dependencies: {}
impl IntoAnyCalendar for Persian { # [inline] fn to_any (self) -> AnyCalendar { AnyCalendar :: Persian (Persian) } # [inline] fn kind (& self) -> AnyCalendarKind { AnyCalendarKind :: Persian } # [inline] fn from_any (any : AnyCalendar) -> Result < Self , AnyCalendar > { if let AnyCalendar :: Persian (cal) = any { Ok (cal) } else { Err (any) } } # [inline] fn from_any_ref (any : & AnyCalendar) -> Option < & Self > { if let AnyCalendar :: Persian (cal) = any { Some (cal) } else { None } } # [inline] fn date_to_any (& self , d : & Self :: DateInner) -> AnyDateInner { AnyDateInner :: Persian (* d) } }
};
}
