// Generated macro for impl_69 (impl)
macro_rules! Depcrate_any_calendarimpl_69 {
() => {
// Module: crate::any_calendar
// Provides: {"impl_69"}
// Dependencies: {}
impl IntoAnyCalendar for Buddhist { # [inline] fn to_any (self) -> AnyCalendar { AnyCalendar :: Buddhist (Buddhist) } # [inline] fn kind (& self) -> AnyCalendarKind { AnyCalendarKind :: Buddhist } # [inline] fn from_any (any : AnyCalendar) -> Result < Self , AnyCalendar > { if let AnyCalendar :: Buddhist (cal) = any { Ok (cal) } else { Err (any) } } # [inline] fn from_any_ref (any : & AnyCalendar) -> Option < & Self > { if let AnyCalendar :: Buddhist (cal) = any { Some (cal) } else { None } } # [inline] fn date_to_any (& self , d : & Self :: DateInner) -> AnyDateInner { AnyDateInner :: Buddhist (* d) } }
};
}
