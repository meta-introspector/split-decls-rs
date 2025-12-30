// Generated macro for impl_83 (impl)
macro_rules! Depcrate_any_calendarimpl_83 {
() => {
// Module: crate::any_calendar
// Provides: {"impl_83"}
// Dependencies: {}
impl IntoAnyCalendar for Indian { # [inline] fn to_any (self) -> AnyCalendar { AnyCalendar :: Indian (Indian) } # [inline] fn kind (& self) -> AnyCalendarKind { AnyCalendarKind :: Indian } # [inline] fn from_any (any : AnyCalendar) -> Result < Self , AnyCalendar > { if let AnyCalendar :: Indian (cal) = any { Ok (cal) } else { Err (any) } } # [inline] fn from_any_ref (any : & AnyCalendar) -> Option < & Self > { if let AnyCalendar :: Indian (cal) = any { Some (cal) } else { None } } # [inline] fn date_to_any (& self , d : & Self :: DateInner) -> AnyDateInner { AnyDateInner :: Indian (* d) } }
};
}
