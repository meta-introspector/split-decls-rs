// Generated macro for impl_64 (impl)
macro_rules! Depcrate_any_calendarimpl_64 {
() => {
// Module: crate::any_calendar
// Provides: {"impl_64"}
// Dependencies: {}
impl AnyCalendarKind { # [doc = " Selects the [`AnyCalendarKind`] appropriate for the given [`CalendarPreferences`]."] pub fn new (prefs : CalendarPreferences) -> Self { if let Some (kind) = prefs . calendar_algorithm . and_then (| a | a . try_into () . ok ()) { return kind ; } match (prefs . calendar_algorithm , prefs . locale_preferences . region () . as_ref () . map (| r | r . as_str ()) ,) { (Some (CalendarAlgorithm :: Hijri (None)) , Some ("AE" | "BH" | "KW" | "QA" | "SA")) => { AnyCalendarKind :: HijriUmmAlQura } (Some (CalendarAlgorithm :: Hijri (None)) , _) => AnyCalendarKind :: HijriTabularTypeIIFriday , (_ , Some ("TH")) => AnyCalendarKind :: Buddhist , (_ , Some ("AF" | "IR")) => AnyCalendarKind :: Persian , _ => AnyCalendarKind :: Gregorian , } } }
};
}
