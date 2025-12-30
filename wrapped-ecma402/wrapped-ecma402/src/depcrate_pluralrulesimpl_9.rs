// Generated macro for impl_9 (impl)
macro_rules! Depcrate_pluralrulesimpl_9 {
() => {
// Module: crate::pluralrules
// Provides: {"impl_9"}
// Dependencies: {}
impl ecma402_traits :: pluralrules :: PluralRules for PluralRules { type Error = icu_provider :: DataError ; fn try_new < L > (l : L , opts : ecma402_traits :: pluralrules :: Options) -> Result < Self , Self :: Error > where L : ecma402_traits :: Locale , Self : Sized , { # [expect (clippy :: unwrap_used)] let locale = icu :: locale :: Locale :: try_from_str (& l . to_string ()) . unwrap () ; let prefs = icu :: plurals :: PluralRulesPreferences :: from (& locale) ; let rule_type = internal :: to_icu4x_type (& opts . in_type) ; let rep = ipr :: PluralRules :: try_new (prefs , rule_type . into ()) ? ; Ok (Self { opts , rep }) } fn select < W > (& self , number : f64 , writer : & mut W) -> std :: fmt :: Result where W : std :: fmt :: Write , { let op = internal :: to_icu4x_operands (number , self . opts . clone ()) ; let result = self . rep . category_for (op) ; write ! (writer , "{}" , internal :: as_str (result)) } }
};
}
