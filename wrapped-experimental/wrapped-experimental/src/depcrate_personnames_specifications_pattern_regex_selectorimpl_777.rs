// Generated macro for impl_777 (impl)
macro_rules! Depcrate_personnames_specifications_pattern_regex_selectorimpl_777 {
() => {
// Module: crate::personnames::specifications::pattern_regex_selector
// Provides: {"impl_777"}
// Dependencies: {}
impl FromStr for NameField { type Err = PersonNamesFormatterError ; fn from_str (value : & str) -> Result < Self , Self :: Err > { let mut value_iter = value . split ('-') ; let name_field_kind = value_iter . next () . map (NameFieldKind :: from_str) . unwrap_or_else (| | { icu_provider :: log :: warn ! ("unable to match") ; Err (PersonNamesFormatterError :: InvalidCldrData) }) ? ; let field_modifier_1 = value_iter . next () . map (FieldModifier :: from_str) . unwrap_or (Ok (FieldModifier :: None)) ? . bit_value () ; let field_modifier_2 = value_iter . next () . map (FieldModifier :: from_str) . unwrap_or (Ok (FieldModifier :: None)) ? . bit_value () ; let field_modifier_3 = value_iter . next () . map (FieldModifier :: from_str) . unwrap_or (Ok (FieldModifier :: None)) ? . bit_value () ; let field_modifier_4 = value_iter . next () . map (FieldModifier :: from_str) . unwrap_or (Ok (FieldModifier :: None)) ? . bit_value () ; Ok (Self { kind : name_field_kind , modifier : FieldModifierSet { value : field_modifier_1 | field_modifier_2 | field_modifier_3 | field_modifier_4 , } , }) } }
};
}
