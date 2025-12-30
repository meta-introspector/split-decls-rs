// Generated macro for impl_71 (impl)
macro_rules! Depcrate_any_calendarimpl_71 {
() => {
// Module: crate::any_calendar
// Provides: {"impl_71"}
// Dependencies: {}
impl IntoAnyCalendar for ChineseTraditional { # [inline] fn to_any (self) -> AnyCalendar { AnyCalendar :: Chinese (self) } # [inline] fn kind (& self) -> AnyCalendarKind { AnyCalendarKind :: Chinese } # [inline] fn from_any (any : AnyCalendar) -> Result < Self , AnyCalendar > { if let AnyCalendar :: Chinese (cal) = any { Ok (cal) } else { Err (any) } } # [inline] fn from_any_ref (any : & AnyCalendar) -> Option < & Self > { if let AnyCalendar :: Chinese (cal) = any { Some (cal) } else { None } } # [inline] fn date_to_any (& self , d : & Self :: DateInner) -> AnyDateInner { AnyDateInner :: Chinese (* d) } }
};
}
