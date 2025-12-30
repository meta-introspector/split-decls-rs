// Generated macro for impl_864 (impl)
macro_rules! Depcrate_raw_neoimpl_864 {
() => {
// Module: crate::raw::neo
// Provides: {"impl_864"}
// Dependencies: {}
impl RawPreferences { # [inline] pub (crate) fn from_prefs (prefs : DateTimeFormatterPreferences) -> Self { Self { hour_cycle : prefs . hour_cycle . map (fields :: Hour :: from_hour_cycle) , } } }
};
}
