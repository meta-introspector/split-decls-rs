// Generated macro for impl_91 (impl)
macro_rules! Depcrate_any_calendarimpl_91 {
() => {
// Module: crate::any_calendar
// Provides: {"impl_91"}
// Dependencies: {}
impl IntoAnyCalendar for Iso { # [inline] fn to_any (self) -> AnyCalendar { AnyCalendar :: Iso (Iso) } # [inline] fn kind (& self) -> AnyCalendarKind { AnyCalendarKind :: Iso } # [inline] fn from_any (any : AnyCalendar) -> Result < Self , AnyCalendar > { if let AnyCalendar :: Iso (cal) = any { Ok (cal) } else { Err (any) } } # [inline] fn from_any_ref (any : & AnyCalendar) -> Option < & Self > { if let AnyCalendar :: Iso (cal) = any { Some (cal) } else { None } } # [inline] fn date_to_any (& self , d : & Self :: DateInner) -> AnyDateInner { AnyDateInner :: Iso (* d) } }
};
}
