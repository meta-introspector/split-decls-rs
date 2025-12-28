macro_rules! deps {
    () => {
        PluralRules!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl Memoizable for PluralRules { type Args = (PluralRuleType ,) ; type Error = & 'static str ; fn construct (lang : LanguageIdentifier , args : Self :: Args) -> Result < Self , Self :: Error > { let default_lang : LanguageIdentifier = "en" . parse () . unwrap () ; let pr_lang = negotiate_languages (& [lang] , & IntlPluralRules :: get_locales (args . 0) , Some (& default_lang) , NegotiationStrategy :: Lookup ,) [0] . clone () ; Ok (Self (IntlPluralRules :: create (pr_lang , args . 0) ?)) } }
    };
}

impl_90!()