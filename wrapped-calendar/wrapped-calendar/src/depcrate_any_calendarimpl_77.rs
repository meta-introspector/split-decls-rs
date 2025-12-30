// Generated macro for impl_77 (impl)
macro_rules! Depcrate_any_calendarimpl_77 {
() => {
// Module: crate::any_calendar
// Provides: {"impl_77"}
// Dependencies: {}
impl IntoAnyCalendar for Ethiopian { # [inline] fn to_any (self) -> AnyCalendar { AnyCalendar :: Ethiopian (self) } # [inline] fn kind (& self) -> AnyCalendarKind { match self . era_style () { EthiopianEraStyle :: AmeteAlem => AnyCalendarKind :: EthiopianAmeteAlem , EthiopianEraStyle :: AmeteMihret => AnyCalendarKind :: Ethiopian , } } # [inline] fn from_any (any : AnyCalendar) -> Result < Self , AnyCalendar > { if let AnyCalendar :: Ethiopian (cal) = any { Ok (cal) } else { Err (any) } } # [inline] fn from_any_ref (any : & AnyCalendar) -> Option < & Self > { if let AnyCalendar :: Ethiopian (cal) = any { Some (cal) } else { None } } # [inline] fn date_to_any (& self , d : & Self :: DateInner) -> AnyDateInner { AnyDateInner :: Ethiopian (* d) } }
};
}
