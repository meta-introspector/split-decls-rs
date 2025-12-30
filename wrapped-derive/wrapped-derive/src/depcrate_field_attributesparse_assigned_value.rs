// Generated macro for parse_assigned_value (function)
macro_rules! Depcrate_field_attributesparse_assigned_value {
() => {
// Module: crate::field_attributes
// Provides: {"parse_assigned_value"}
// Dependencies: {}
fn parse_assigned_value (opt_name : & str , mut tokens_iter : impl Iterator < Item = TokenTree > , default_span : Span ,) -> Result < TokenStream > { let eq_sign = tokens_iter . next () . ok_or_else (| | { let msg = format ! ("Invalid syntax for #[{ARBITRARY_ATTRIBUTE_NAME}], `{opt_name}` is missing assignment.") ; syn :: Error :: new (default_span , msg) }) ? ; if eq_sign . to_string () == "=" { Ok (tokens_iter . collect ()) } else { let msg = format ! ("Invalid syntax for #[{ARBITRARY_ATTRIBUTE_NAME}], expected `=` after `{opt_name}`, got: `{eq_sign}`") ; Err (syn :: Error :: new (eq_sign . span () , msg)) } }
};
}
