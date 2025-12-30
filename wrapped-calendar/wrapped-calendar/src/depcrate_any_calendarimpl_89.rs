// Generated macro for impl_89 (impl)
macro_rules! Depcrate_any_calendarimpl_89 {
() => {
// Module: crate::any_calendar
// Provides: {"impl_89"}
// Dependencies: {}
impl IntoAnyCalendar for Hijri < hijri :: UmmAlQura > { # [inline] fn to_any (self) -> AnyCalendar { AnyCalendar :: HijriUmmAlQura (self) } # [inline] fn kind (& self) -> AnyCalendarKind { AnyCalendarKind :: HijriUmmAlQura } # [inline] fn from_any (any : AnyCalendar) -> Result < Self , AnyCalendar > { if let AnyCalendar :: HijriUmmAlQura (cal) = any { Ok (cal) } else { Err (any) } } # [inline] fn from_any_ref (any : & AnyCalendar) -> Option < & Self > { if let AnyCalendar :: HijriUmmAlQura (cal) = any { Some (cal) } else { None } } # [inline] fn date_to_any (& self , d : & Self :: DateInner) -> AnyDateInner { AnyDateInner :: HijriUmmAlQura (* d) } }
};
}
