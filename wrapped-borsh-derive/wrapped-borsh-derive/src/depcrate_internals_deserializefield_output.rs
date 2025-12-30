// Generated macro for field_output (function)
macro_rules! Depcrate_internals_deserializefield_output {
() => {
// Module: crate::internals::deserialize
// Provides: {"field_output"}
// Dependencies: {}
# [doc = " function which computes derive output [proc_macro2::TokenStream]"] # [doc = " of code, which deserializes single field"] fn field_output (field_name : Option < & Ident > , cratename : & Path , deserialize_with : Option < ExprPath > ,) -> TokenStream2 { let default_path : ExprPath = syn :: parse2 (quote ! { # cratename :: BorshDeserialize :: deserialize_reader }) . unwrap () ; let path : ExprPath = deserialize_with . unwrap_or (default_path) ; if let Some (field_name) = field_name { quote ! { # field_name : # path (reader) ?, } } else { quote ! { # path (reader) ?, } } }
};
}
