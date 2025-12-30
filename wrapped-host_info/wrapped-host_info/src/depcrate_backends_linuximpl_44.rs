// Generated macro for impl_44 (impl)
macro_rules! Depcrate_backends_linuximpl_44 {
() => {
// Module: crate::backends::linux
// Provides: {"impl_44"}
// Dependencies: {}
impl HostInfoBackend for LinuxHostInfoBackend { # [cfg (feature = "datetime")] fn datetime_preferences () -> Result < icu_datetime :: DateTimeFormatterPreferences , HostInfoError > { use crate :: posix :: { raw_locale_categories , LocaleCategory } ; let mut categories = raw_locale_categories () ? ; let mut locale = Locale :: UNKNOWN ; if let Some (lc_time) = categories . remove (& LocaleCategory :: Time) { if let Ok (loc) = PosixLocale :: try_from_str (& lc_time) { if let Ok (loc) = Locale :: try_from (loc) { locale = loc ; } } } else { if let Some (lc_all) = categories . remove (& LocaleCategory :: All) { if let Ok (loc) = PosixLocale :: try_from_str (& lc_all) { if let Ok (loc) = Locale :: try_from (loc) { locale = loc ; } } } } let mut result = icu_datetime :: DateTimeFormatterPreferences :: from (locale) ; result . numbering_system = None ; result . hour_cycle = Self :: hour_cycle () ? ; result . calendar_algorithm = Self :: calendar () ? ; Ok (result) } fn requested_locales () -> Result < Vec < Locale > , HostInfoError > { Ok (Self :: raw_requested_locales () ? . into_iter () . filter_map (| s | { PosixLocale :: try_from_str (& s) . ok () . and_then (| posix_locale | Locale :: try_from (posix_locale) . ok ()) }) . collect ()) } fn hour_cycle () -> Result < Option < HourCycle > , HostInfoError > { # [cfg (feature = "gnome")] if let Some (hc) = gnome_clock_format_hc () { return Ok (Some (hc)) ; } Ok (None) } }
};
}
