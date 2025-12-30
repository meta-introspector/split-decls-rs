// Generated macro for impl_216 (impl)
macro_rules! Depcrate_optionsimpl_216 {
() => {
// Module: crate::options
// Provides: {"impl_216"}
// Dependencies: {}
impl From < ResolvedCollatorOptions > for CollatorPreferences { # [doc = " Convenience conversion for copying the preferences from an"] # [doc = " existing collator into a new one."] # [doc = ""] # [doc = " Note that some preferences may not be fully preserved when recovering them"] # [doc = " from an already initialized collator e.g [`LocalePreferences`] and [`CollationType`], because"] # [doc = " those are only relevant when loading the collation data."] # [doc = ""] # [doc = " [`LocalePreferences`]: icu_locale_core::preferences::LocalePreferences"] # [doc = " [`CollationType`]: crate::preferences::CollationType"] fn from (options : ResolvedCollatorOptions) -> CollatorPreferences { CollatorPreferences { case_first : Some (options . case_first) , numeric_ordering : Some (options . numeric) , .. Default :: default () } } }
};
}
