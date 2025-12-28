macro_rules! deps {
    () => {
        IntlMemoizer!();
        Memoizable!();
        IntlLangMemoizer!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use super :: * ; use fluent_langneg :: { negotiate_languages , NegotiationStrategy } ; use intl_pluralrules :: { PluralCategory , PluralRuleType , PluralRules as IntlPluralRules } ; use std :: { sync :: Arc , thread } ; struct PluralRules (pub IntlPluralRules) ; impl PluralRules { pub fn new (lang : LanguageIdentifier , pr_type : PluralRuleType ,) -> Result < Self , & 'static str > { let default_lang : LanguageIdentifier = "en" . parse () . unwrap () ; let pr_lang = negotiate_languages (& [lang] , & IntlPluralRules :: get_locales (pr_type) , Some (& default_lang) , NegotiationStrategy :: Lookup ,) [0] . clone () ; Ok (Self (IntlPluralRules :: create (pr_lang , pr_type) ?)) } } impl Memoizable for PluralRules { type Args = (PluralRuleType ,) ; type Error = & 'static str ; fn construct (lang : LanguageIdentifier , args : Self :: Args) -> Result < Self , Self :: Error > { Self :: new (lang , args . 0) } } # [test] fn test_single_thread () { let lang : LanguageIdentifier = "en" . parse () . unwrap () ; let mut memoizer = IntlMemoizer :: default () ; { let en_memoizer = memoizer . get_for_lang (lang . clone ()) ; let result = en_memoizer . with_try_get :: < PluralRules , _ , _ > ((PluralRuleType :: CARDINAL ,) , | cb | cb . 0 . select (5)) . unwrap () ; assert_eq ! (result , Ok (PluralCategory :: OTHER)) ; } { let en_memoizer = memoizer . get_for_lang (lang) ; let result = en_memoizer . with_try_get :: < PluralRules , _ , _ > ((PluralRuleType :: CARDINAL ,) , | cb | cb . 0 . select (5)) . unwrap () ; assert_eq ! (result , Ok (PluralCategory :: OTHER)) ; } } # [test] fn test_concurrent () { let lang : LanguageIdentifier = "en" . parse () . unwrap () ; let memoizer = Arc :: new (concurrent :: IntlLangMemoizer :: new (lang)) ; let mut threads = vec ! [] ; for _ in 0 .. 4 { let memoizer = Arc :: clone (& memoizer) ; threads . push (thread :: spawn (move | | { memoizer . with_try_get :: < PluralRules , _ , _ > ((PluralRuleType :: CARDINAL ,) , | cb | { cb . 0 . select (5) }) . expect ("Failed to get a PluralRules result.") })) ; } for thread in threads . drain (..) { let result = thread . join () . expect ("Failed to join thread.") ; assert_eq ! (result , Ok (PluralCategory :: OTHER)) ; } } }
    };
}

tests!()