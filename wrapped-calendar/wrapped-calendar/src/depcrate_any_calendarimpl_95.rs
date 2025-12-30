// Generated macro for impl_95 (impl)
macro_rules! Depcrate_any_calendarimpl_95 {
() => {
// Module: crate::any_calendar
// Provides: {"impl_95"}
// Dependencies: {}
impl IntoAnyCalendar for JapaneseExtended { # [inline] fn to_any (self) -> AnyCalendar { AnyCalendar :: JapaneseExtended (self) } # [inline] fn kind (& self) -> AnyCalendarKind { AnyCalendarKind :: JapaneseExtended } # [inline] fn from_any (any : AnyCalendar) -> Result < Self , AnyCalendar > { if let AnyCalendar :: JapaneseExtended (cal) = any { Ok (cal) } else { Err (any) } } # [inline] fn from_any_ref (any : & AnyCalendar) -> Option < & Self > { if let AnyCalendar :: JapaneseExtended (cal) = any { Some (cal) } else { None } } # [inline] fn date_to_any (& self , d : & Self :: DateInner) -> AnyDateInner { AnyDateInner :: JapaneseExtended (* d) } }
};
}
