// Generated macro for impl_87 (impl)
macro_rules! Depcrate_any_calendarimpl_87 {
() => {
// Module: crate::any_calendar
// Provides: {"impl_87"}
// Dependencies: {}
impl IntoAnyCalendar for Hijri < hijri :: AstronomicalSimulation > { # [inline] fn to_any (self) -> AnyCalendar { AnyCalendar :: HijriSimulated (self) } # [inline] fn kind (& self) -> AnyCalendarKind { match self . 0 . location { crate :: cal :: hijri_internal :: SimulatedLocation :: Mecca => { AnyCalendarKind :: HijriSimulatedMecca } } } # [inline] fn from_any (any : AnyCalendar) -> Result < Self , AnyCalendar > { if let AnyCalendar :: HijriSimulated (cal) = any { Ok (cal) } else { Err (any) } } # [inline] fn from_any_ref (any : & AnyCalendar) -> Option < & Self > { if let AnyCalendar :: HijriSimulated (cal) = any { Some (cal) } else { None } } # [inline] fn date_to_any (& self , d : & Self :: DateInner) -> AnyDateInner { AnyDateInner :: HijriSimulated (* d) } }
};
}
