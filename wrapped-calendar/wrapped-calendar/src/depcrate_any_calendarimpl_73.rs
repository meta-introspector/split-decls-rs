// Generated macro for impl_73 (impl)
macro_rules! Depcrate_any_calendarimpl_73 {
() => {
// Module: crate::any_calendar
// Provides: {"impl_73"}
// Dependencies: {}
impl IntoAnyCalendar for Coptic { # [inline] fn to_any (self) -> AnyCalendar { AnyCalendar :: Coptic (Coptic) } # [inline] fn kind (& self) -> AnyCalendarKind { AnyCalendarKind :: Coptic } # [inline] fn from_any (any : AnyCalendar) -> Result < Self , AnyCalendar > { if let AnyCalendar :: Coptic (cal) = any { Ok (cal) } else { Err (any) } } # [inline] fn from_any_ref (any : & AnyCalendar) -> Option < & Self > { if let AnyCalendar :: Coptic (cal) = any { Some (cal) } else { None } } # [inline] fn date_to_any (& self , d : & Self :: DateInner) -> AnyDateInner { AnyDateInner :: Coptic (* d) } }
};
}
