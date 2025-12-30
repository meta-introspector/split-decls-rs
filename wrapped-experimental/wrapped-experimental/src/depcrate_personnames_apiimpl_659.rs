// Generated macro for impl_659 (impl)
macro_rules! Depcrate_personnames_apiimpl_659 {
() => {
// Module: crate::personnames::api
// Provides: {"impl_659"}
// Dependencies: {}
impl PersonNamesFormatterOptions { # [cfg (feature = "compiled_data")] pub fn new (target_locale : Locale , order : FormattingOrder , length : FormattingLength , usage : FormattingUsage , formality : FormattingFormality ,) -> Self { let lc = icu_locale :: LocaleExpander :: new_extended () ; let mut final_locale = target_locale . clone () ; lc . maximize (& mut final_locale . id) ; Self { target_locale : final_locale , order , length , usage , formality , } } }
};
}
