// Generated macro for impl_55 (impl)
macro_rules! Depcrate_any_calendarimpl_55 {
() => {
// Module: crate::any_calendar
// Provides: {"impl_55"}
// Dependencies: {}
impl PartialOrd for AnyDateInner { # [rustfmt :: skip] fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { use AnyDateInner :: * ; match (self , other) { (Buddhist (d1) , Buddhist (d2)) => d1 . partial_cmp (d2) , (Chinese (d1) , Chinese (d2)) => d1 . partial_cmp (d2) , (Coptic (d1) , Coptic (d2)) => d1 . partial_cmp (d2) , (Dangi (d1) , Dangi (d2)) => d1 . partial_cmp (d2) , (Ethiopian (d1) , Ethiopian (d2)) => d1 . partial_cmp (d2) , (Gregorian (d1) , Gregorian (d2)) => d1 . partial_cmp (d2) , (Hebrew (d1) , Hebrew (d2)) => d1 . partial_cmp (d2) , (HijriSimulated (d1) , HijriSimulated (d2)) => d1 . partial_cmp (d2) , (& HijriTabular (ref d1 , s1) , & HijriTabular (ref d2 , s2)) if s1 == s2 => d1 . partial_cmp (d2) , (HijriUmmAlQura (d1) , HijriUmmAlQura (d2)) => d1 . partial_cmp (d2) , (Indian (d1) , Indian (d2)) => d1 . partial_cmp (d2) , (Iso (d1) , Iso (d2)) => d1 . partial_cmp (d2) , (Japanese (d1) , Japanese (d2)) => d1 . partial_cmp (d2) , (JapaneseExtended (d1) , JapaneseExtended (d2)) => d1 . partial_cmp (d2) , (Persian (d1) , Persian (d2)) => d1 . partial_cmp (d2) , (Roc (d1) , Roc (d2)) => d1 . partial_cmp (d2) , _ => None , } } }
};
}
