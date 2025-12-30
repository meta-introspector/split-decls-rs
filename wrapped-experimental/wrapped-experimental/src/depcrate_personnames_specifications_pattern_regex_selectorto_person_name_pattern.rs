// Generated macro for to_person_name_pattern (function)
macro_rules! Depcrate_personnames_specifications_pattern_regex_selectorto_person_name_pattern {
() => {
// Module: crate::personnames::specifications::pattern_regex_selector
// Provides: {"to_person_name_pattern"}
// Dependencies: {}
pub fn to_person_name_pattern (value : & str ,) -> Result < PersonNamePattern < '_ > , PersonNamesFormatterError > { let mut name_fields_map : Vec < (NameField , Cow < str >) > = Vec :: new () ; let parsed_pattern = MultiNamedPlaceholderPattern :: try_from_str (value , Default :: default ()) ? ; let mut current_name_field = None ; let mut current_literal = None ; for item in parsed_pattern . iter () { match item { PatternItem :: Literal (s) => { debug_assert ! (current_literal . is_none ()) ; current_literal = Some (s) ; } PatternItem :: Placeholder (key) => { if let Some (name_field) = current_name_field . take () { let trailing = Cow :: Owned (String :: from (current_literal . take () . unwrap_or (""))) ; name_fields_map . push ((name_field , trailing)) ; } current_name_field = Some (NameField :: from_str (key . 0) ?) ; } } } if let Some (name_field) = current_name_field . take () { let trailing = Cow :: Owned (String :: from (current_literal . take () . unwrap_or (""))) ; name_fields_map . push ((name_field , trailing)) ; } Ok (PersonNamePattern { name_fields : name_fields_map , }) }
};
}
