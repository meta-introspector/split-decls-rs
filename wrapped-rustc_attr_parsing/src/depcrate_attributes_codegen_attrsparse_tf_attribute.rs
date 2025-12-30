// Generated macro for parse_tf_attribute (function)
macro_rules! Depcrate_attributes_codegen_attrsparse_tf_attribute {
() => {
// Module: crate::attributes::codegen_attrs
// Provides: {"parse_tf_attribute"}
// Dependencies: {}
fn parse_tf_attribute < 'c , S : Stage > (cx : & 'c mut AcceptContext < '_ , '_ , S > , args : & 'c ArgParser < '_ > ,) -> impl IntoIterator < Item = (Symbol , Span) > + 'c { let mut features = Vec :: new () ; let ArgParser :: List (list) = args else { cx . expected_list (cx . attr_span) ; return features ; } ; if list . is_empty () { cx . warn_empty_attribute (cx . attr_span) ; return features ; } for item in list . mixed () { let Some (name_value) = item . meta_item () else { cx . expected_name_value (item . span () , Some (sym :: enable)) ; return features ; } ; let Some (name) = name_value . path () . word_sym () else { cx . expected_name_value (name_value . path () . span () , Some (sym :: enable)) ; return features ; } ; if name != sym :: enable { cx . expected_name_value (name_value . path () . span () , Some (sym :: enable)) ; return features ; } let Some (name_value) = name_value . args () . name_value () else { cx . expected_name_value (item . span () , Some (sym :: enable)) ; return features ; } ; let Some (value_str) = name_value . value_as_str () else { cx . expected_string_literal (name_value . value_span , Some (name_value . value_as_lit ())) ; return features ; } ; for feature in value_str . as_str () . split (",") { features . push ((Symbol :: intern (feature) , item . span ())) ; } } features }
};
}
