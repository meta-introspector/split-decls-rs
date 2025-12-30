// Generated macro for impl_75 (impl)
macro_rules! Depcrate_any_calendarimpl_75 {
() => {
// Module: crate::any_calendar
// Provides: {"impl_75"}
// Dependencies: {}
impl IntoAnyCalendar for KoreanTraditional { # [inline] fn to_any (self) -> AnyCalendar { AnyCalendar :: Dangi (self) } # [inline] fn kind (& self) -> AnyCalendarKind { AnyCalendarKind :: Dangi } # [inline] fn from_any (any : AnyCalendar) -> Result < Self , AnyCalendar > { if let AnyCalendar :: Dangi (cal) = any { Ok (cal) } else { Err (any) } } # [inline] fn from_any_ref (any : & AnyCalendar) -> Option < & Self > { if let AnyCalendar :: Dangi (cal) = any { Some (cal) } else { None } } # [inline] fn date_to_any (& self , d : & Self :: DateInner) -> AnyDateInner { AnyDateInner :: Dangi (* d) } }
};
}
