// Generated macro for impl_775 (impl)
macro_rules! Depcrate_personnames_specifications_pattern_regex_selectorimpl_775 {
() => {
// Module: crate::personnames::specifications::pattern_regex_selector
// Provides: {"impl_775"}
// Dependencies: {}
impl FromStr for NameFieldKind { type Err = PersonNamesFormatterError ; fn from_str (value : & str) -> Result < Self , Self :: Err > { match value { "title" => Ok (NameFieldKind :: Title) , "given" => Ok (NameFieldKind :: Given) , "given2" => Ok (NameFieldKind :: Given2) , "surname" => Ok (NameFieldKind :: Surname) , "surname2" => Ok (NameFieldKind :: Surname2) , "generation" => Ok (NameFieldKind :: Generation) , "credentials" => Ok (NameFieldKind :: Credentials) , _ => { icu_provider :: log :: warn ! ("Invalid NameFieldKind value matched [{value}]") ; Err (PersonNamesFormatterError :: InvalidCldrData) } } } }
};
}
