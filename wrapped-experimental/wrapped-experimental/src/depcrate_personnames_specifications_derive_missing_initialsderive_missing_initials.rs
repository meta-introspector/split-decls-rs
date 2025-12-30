// Generated macro for derive_missing_initials (function)
macro_rules! Depcrate_personnames_specifications_derive_missing_initialsderive_missing_initials {
() => {
// Module: crate::personnames::specifications::derive_missing_initials
// Provides: {"derive_missing_initials"}
// Dependencies: {}
# [doc = ""] # [doc = " Derive missing initials from the name"] # [doc = ""] # [doc = " https://www.unicode.org/reports/tr35/tr35-personNames.html#derive-initials"] pub fn derive_missing_initials (person_name : & dyn PersonName , requested_field : & NameField , initial_pattern_str : & str , initial_sequence_pattern_str : & str ,) -> String { let initial_pattern = SinglePlaceholderPattern :: try_from_str (initial_pattern_str , Default :: default ()) . unwrap () ; let initial_sequence_pattern = DoublePlaceholderPattern :: try_from_str (initial_sequence_pattern_str , Default :: default ()) . unwrap () ; if person_name . has_name_field (requested_field) { return String :: from (person_name . get (requested_field)) ; } if requested_field . modifier . has_field (FieldModifier :: Initial) { let initials = person_name . get (& NameField { kind : requested_field . kind , modifier : requested_field . modifier . with_length (FieldLength :: Auto) , }) . split (' ') . filter_map (| s | s . trim () . chars () . next ()) ; let mut interpolated_initials = initials . map (| initial | initial_pattern . interpolate ((initial ,))) ; let mut output = interpolated_initials . next () . map (| s | s . write_to_string () . into_owned ()) . unwrap_or (String :: new ()) ; for s in interpolated_initials { output = initial_sequence_pattern . interpolate ((output , s . write_to_string () . into_owned ())) . write_to_string () . into_owned () ; } return output ; } String :: from ("") }
};
}
