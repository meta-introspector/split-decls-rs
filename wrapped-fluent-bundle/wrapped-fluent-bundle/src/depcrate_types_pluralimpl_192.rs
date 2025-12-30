// Generated macro for impl_192 (impl)
macro_rules! Depcrate_types_pluralimpl_192 {
() => {
// Module: crate::types::plural
// Provides: {"impl_192"}
// Dependencies: {}
impl Memoizable for PluralRules { type Args = (PluralRuleType ,) ; type Error = & 'static str ; fn construct (lang : LanguageIdentifier , args : Self :: Args) -> Result < Self , Self :: Error > { let default_lang : LanguageIdentifier = "en" . parse () . unwrap () ; let pr_lang = negotiate_languages (& [lang] , & IntlPluralRules :: get_locales (args . 0) , Some (& default_lang) , NegotiationStrategy :: Lookup ,) [0] . clone () ; Ok (Self (IntlPluralRules :: create (pr_lang , args . 0) ?)) } }
};
}
