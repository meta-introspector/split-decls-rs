// Generated macro for impl_490 (impl)
macro_rules! Depcrate_preferences_localeimpl_490 {
() => {
// Module: crate::preferences::locale
// Provides: {"impl_490"}
// Dependencies: {}
# [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] impl From < LocalePreferences > for crate :: Locale { fn from (prefs : LocalePreferences) -> Self { Self { id : crate :: LanguageIdentifier { language : prefs . language , script : prefs . script , region : prefs . region , variants : prefs . variant . map (Variants :: from_variant) . unwrap_or_default () , } , extensions : { let mut extensions = crate :: extensions :: Extensions :: default () ; if let Some (sd) = prefs . subdivision { extensions . unicode . keywords . set (crate :: extensions :: unicode :: key ! ("sd") , crate :: extensions :: unicode :: Value :: from_subtag (Some (sd)) ,) ; } if let Some (rg) = prefs . ue_region { # [expect (clippy :: unwrap_used)] extensions . unicode . keywords . set (crate :: extensions :: unicode :: key ! ("rg") , crate :: extensions :: unicode :: Value :: try_from_str (rg . as_str ()) . unwrap () ,) ; } extensions } , } } }
};
}
