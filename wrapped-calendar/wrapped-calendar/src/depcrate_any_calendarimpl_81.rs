// Generated macro for impl_81 (impl)
macro_rules! Depcrate_any_calendarimpl_81 {
() => {
// Module: crate::any_calendar
// Provides: {"impl_81"}
// Dependencies: {}
impl IntoAnyCalendar for Hebrew { # [inline] fn to_any (self) -> AnyCalendar { AnyCalendar :: Hebrew (Hebrew) } # [inline] fn kind (& self) -> AnyCalendarKind { AnyCalendarKind :: Hebrew } # [inline] fn from_any (any : AnyCalendar) -> Result < Self , AnyCalendar > { if let AnyCalendar :: Hebrew (cal) = any { Ok (cal) } else { Err (any) } } # [inline] fn from_any_ref (any : & AnyCalendar) -> Option < & Self > { if let AnyCalendar :: Hebrew (cal) = any { Some (cal) } else { None } } # [inline] fn date_to_any (& self , d : & Self :: DateInner) -> AnyDateInner { AnyDateInner :: Hebrew (* d) } }
};
}
