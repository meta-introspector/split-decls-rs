// Generated macro for impl_85 (impl)
macro_rules! Depcrate_any_calendarimpl_85 {
() => {
// Module: crate::any_calendar
// Provides: {"impl_85"}
// Dependencies: {}
impl IntoAnyCalendar for Hijri < hijri :: TabularAlgorithm > { # [inline] fn to_any (self) -> AnyCalendar { AnyCalendar :: HijriTabular (self) } # [inline] fn kind (& self) -> AnyCalendarKind { match self . 0 { hijri :: TabularAlgorithm { leap_years : hijri :: TabularAlgorithmLeapYears :: TypeII , epoch : hijri :: TabularAlgorithmEpoch :: Friday , } => AnyCalendarKind :: HijriTabularTypeIIFriday , hijri :: TabularAlgorithm { leap_years : hijri :: TabularAlgorithmLeapYears :: TypeII , epoch : hijri :: TabularAlgorithmEpoch :: Thursday , } => AnyCalendarKind :: HijriTabularTypeIIThursday , } } # [inline] fn from_any (any : AnyCalendar) -> Result < Self , AnyCalendar > { if let AnyCalendar :: HijriTabular (cal) = any { Ok (cal) } else { Err (any) } } # [inline] fn from_any_ref (any : & AnyCalendar) -> Option < & Self > { if let AnyCalendar :: HijriTabular (cal) = any { Some (cal) } else { None } } # [inline] fn date_to_any (& self , d : & Self :: DateInner) -> AnyDateInner { AnyDateInner :: HijriTabular (* d , self . 0) } }
};
}
