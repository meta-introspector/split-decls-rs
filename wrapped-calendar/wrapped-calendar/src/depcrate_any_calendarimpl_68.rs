// Generated macro for impl_68 (impl)
macro_rules! Depcrate_any_calendarimpl_68 {
() => {
// Module: crate::any_calendar
// Provides: {"impl_68"}
// Dependencies: {}
impl IntoAnyCalendar for AnyCalendar { # [inline] fn to_any (self) -> AnyCalendar { self } # [inline] fn kind (& self) -> AnyCalendarKind { self . kind () } # [inline] fn from_any (any : AnyCalendar) -> Result < Self , AnyCalendar > { Ok (any) } # [inline] fn from_any_ref (any : & AnyCalendar) -> Option < & Self > { Some (any) } # [inline] fn date_to_any (& self , d : & Self :: DateInner) -> AnyDateInner { * d } }
};
}
