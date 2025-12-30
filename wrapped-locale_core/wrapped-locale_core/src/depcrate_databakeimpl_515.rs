// Generated macro for impl_515 (impl)
macro_rules! Depcrate_databakeimpl_515 {
() => {
// Module: crate::databake
// Provides: {"impl_515"}
// Dependencies: {}
impl Bake for LanguageIdentifier { fn bake (& self , env : & CrateEnv) -> TokenStream { env . insert ("icu_locale_core") ; let repr = self . to_string () ; if self . variants . len () <= 1 { quote ! { icu_locale_core :: langid ! (# repr) } } else { quote ! { icu_locale_core :: LanguageIdentifier :: try_from_str (# repr) . unwrap () } } } }
};
}
