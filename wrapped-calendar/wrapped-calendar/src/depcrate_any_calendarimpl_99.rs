// Generated macro for impl_99 (impl)
macro_rules! Depcrate_any_calendarimpl_99 {
() => {
// Module: crate::any_calendar
// Provides: {"impl_99"}
// Dependencies: {}
impl IntoAnyCalendar for Roc { # [inline] fn to_any (self) -> AnyCalendar { AnyCalendar :: Roc (Roc) } # [inline] fn kind (& self) -> AnyCalendarKind { AnyCalendarKind :: Roc } # [inline] fn from_any (any : AnyCalendar) -> Result < Self , AnyCalendar > { if let AnyCalendar :: Roc (cal) = any { Ok (cal) } else { Err (any) } } # [inline] fn from_any_ref (any : & AnyCalendar) -> Option < & Self > { if let AnyCalendar :: Roc (cal) = any { Some (cal) } else { None } } # [inline] fn date_to_any (& self , d : & Self :: DateInner) -> AnyDateInner { AnyDateInner :: Roc (* d) } }
};
}
