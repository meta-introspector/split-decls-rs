// Generated macro for impl_776 (impl)
macro_rules! Depcrate_personnames_specifications_pattern_regex_selectorimpl_776 {
() => {
// Module: crate::personnames::specifications::pattern_regex_selector
// Provides: {"impl_776"}
// Dependencies: {}
impl FromStr for FieldModifier { type Err = PersonNamesFormatterError ; fn from_str (value : & str) -> Result < Self , Self :: Err > { match value { "informal" => Ok (FieldModifier :: Informal) , "prefix" => Ok (FieldModifier :: Prefix) , "core" => Ok (FieldModifier :: Core) , "allCaps" => Ok (FieldModifier :: AllCaps) , "initialCap" => Ok (FieldModifier :: InitialCap) , "initial" => Ok (FieldModifier :: Initial) , "monogram" => Ok (FieldModifier :: Monogram) , _ => { icu_provider :: log :: warn ! ("Invalid FieldModifier value matched [{value}]") ; Err (PersonNamesFormatterError :: InvalidCldrData) } } } }
};
}
