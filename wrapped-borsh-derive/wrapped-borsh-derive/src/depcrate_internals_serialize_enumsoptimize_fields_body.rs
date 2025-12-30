// Generated macro for optimize_fields_body (function)
macro_rules! Depcrate_internals_serialize_enumsoptimize_fields_body {
() => {
// Module: crate::internals::serialize::enums
// Provides: {"optimize_fields_body"}
// Dependencies: {}
fn optimize_fields_body (fields_body : TokenStream2 , has_unit_variant : bool) -> TokenStream2 { if fields_body . is_empty () { fields_body } else { let unit_fields_catchall = if has_unit_variant { quote ! (_ => { }) } else { TokenStream2 :: new () } ; quote ! (match self { # fields_body # unit_fields_catchall }) } }
};
}
