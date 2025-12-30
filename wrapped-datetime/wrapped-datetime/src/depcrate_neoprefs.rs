// Generated macro for prefs (function)
macro_rules! Depcrate_neoprefs {
() => {
// Module: crate::neo
// Provides: {"prefs"}
// Dependencies: {}
# [test] fn prefs () { use icu_locale :: locale ; assert_eq ! (DateTimeFormatterPreferences :: from_locale_strict (& locale ! ("en-US-u-hc-h23")) . unwrap () . hour_cycle , Some (HourCycle :: H23)) ; assert_eq ! (DateTimeFormatterPreferences :: from_locale_strict (& locale ! ("en-US-u-hc-h24")) . unwrap_err () . hour_cycle , None) ; }
};
}
