// Generated macro for adjust_pattern_field_lengths (function)
macro_rules! Depcrate_provider_skeleton_helpersadjust_pattern_field_lengths {
() => {
// Module: crate::provider::skeleton::helpers
// Provides: {"adjust_pattern_field_lengths"}
// Dependencies: {}
# [doc = " Alters given Pattern so that its fields have the same length as 'fields'."] # [doc = ""] # [doc = "  For example the \"d MMM y\" pattern will be changed to \"d MMMM y\" given fields [\"y\", \"MMMM\", \"d\"]."] fn adjust_pattern_field_lengths (fields : & [Field] , pattern : & mut runtime :: Pattern) { runtime :: helpers :: maybe_replace (pattern , | item | { if let PatternItem :: Field (pattern_field) = item { if let Some (requested_field) = fields . iter () . find (| field | field . symbol . skeleton_cmp (pattern_field . symbol) . is_eq ()) { if requested_field . length != pattern_field . length && requested_field . get_length_type () == pattern_field . get_length_type () { let length = requested_field . length ; let length = if requested_field . symbol . is_at_least_abbreviated () { length . numeric_to_abbr () } else { length } ; return Some (PatternItem :: Field (Field { length , .. * pattern_field })) ; } } } None }) }
};
}
