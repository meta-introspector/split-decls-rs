// Generated macro for field_declaration_output (function)
macro_rules! Depcrate_internals_schema_structsfield_declaration_output {
() => {
// Module: crate::internals::schema::structs
// Provides: {"field_declaration_output"}
// Dependencies: {}
# [doc = " function which computes derive output [proc_macro2::TokenStream]"] # [doc = " of code, which computes declaration of a single field, which is later added to"] # [doc = " the struct's definition as a whole  "] fn field_declaration_output (field_name : Option < & Ident > , field_type : & Type , cratename : & Path , declaration_override : Option < ExprPath > ,) -> TokenStream2 { let default_path : ExprPath = syn :: parse2 (quote ! { <# field_type as # cratename :: BorshSchema >:: declaration }) . unwrap () ; let path = declaration_override . unwrap_or (default_path) ; if let Some (field_name) = field_name { let field_name = field_name . to_token_stream () . to_string () ; quote ! { (# field_name . to_string () , # path ()) } } else { quote ! { # path () } } }
};
}
