// Generated macro for impl_69 (impl)
macro_rules! Depcrate_fallback_algorithmsimpl_69 {
() => {
// Module: crate::fallback::algorithms
// Provides: {"impl_69"}
// Dependencies: {}
impl LocaleFallbackerWithConfig < '_ > { pub (crate) fn normalize (& self , locale : & mut DataLocale , default_script : & mut Option < Script >) { if let Some (subdivision) = locale . subdivision . take () { if let Some (region) = locale . region { if subdivision . as_str () . starts_with (region . to_tinystr () . to_ascii_lowercase () . as_str ()) { locale . subdivision = Some (subdivision) ; } } } let language = locale . language ; if self . config . priority == LocaleFallbackPriority :: Region && locale . region . is_none () { if let Some (script) = locale . script { locale . region = self . likely_subtags . language_script . get (& (language . to_tinystr () . to_unvalidated () , script . to_tinystr () . to_unvalidated () ,)) . copied () ; } if locale . region . is_none () { locale . region = self . likely_subtags . language . get_copied (& language . to_tinystr () . to_unvalidated ()) . map (| (_s , r) | r) ; } } if locale . script . is_some () || self . config . priority == LocaleFallbackPriority :: Script { * default_script = locale . region . and_then (| region | { self . likely_subtags . language_region . get_copied (& (language . to_tinystr () . to_unvalidated () , region . to_tinystr () . to_unvalidated () ,)) }) . or_else (| | { self . likely_subtags . language . get_copied (& language . to_tinystr () . to_unvalidated ()) . map (| (s , _r) | s) }) ; if locale . script == * default_script { locale . script = None ; } } } }
};
}
