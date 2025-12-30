// Generated macro for impl_23 (impl)
macro_rules! Depcrate_backends_androidimpl_23 {
() => {
// Module: crate::backends::android
// Provides: {"impl_23"}
// Dependencies: {}
impl RawHostInfoBackend for AndroidHostInfoBackend { fn raw_requested_locales () -> Result < Vec < String > , HostInfoError > { let mut categories = raw_locale_categories () ? ; let mut locales = Vec :: with_capacity (categories . len ()) ; if let Some (primary_locale) = categories . remove (& LocaleCategory :: All) { locales . push (primary_locale) ; } for s in categories . into_values () { if ! locales . contains (& s) { locales . push (s) ; } } Ok (locales) } }
};
}
