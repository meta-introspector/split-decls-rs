// Generated macro for impl_36 (impl)
macro_rules! Depcrate_external_loadersimpl_36 {
() => {
// Module: crate::external_loaders
// Provides: {"impl_36"}
// Dependencies: {}
impl < P > FormattableAnyCalendarLoader for ExternalLoaderUnstable < '_ , P > where P : DataProvider < icu_calendar :: provider :: CalendarJapaneseModernV1 > + ? Sized , { # [inline] fn load (& self , kind : FormattableAnyCalendarKind) -> Result < FormattableAnyCalendar , DataError > { FormattableAnyCalendar :: try_new_unstable (self . 0 , kind) } }
};
}
