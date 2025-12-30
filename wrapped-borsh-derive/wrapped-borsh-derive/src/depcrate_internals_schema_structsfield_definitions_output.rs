// Generated macro for field_definitions_output (function)
macro_rules! Depcrate_internals_schema_structsfield_definitions_output {
() => {
// Module: crate::internals::schema::structs
// Provides: {"field_definitions_output"}
// Dependencies: {}
# [doc = " function which computes derive output [proc_macro2::TokenStream]"] # [doc = " of code, which adds definitions of a field to the output `definitions: &mut BTreeMap`"] fn field_definitions_output (field_type : & Type , cratename : & Path , definitions_override : Option < ExprPath > ,) -> TokenStream2 { let default_path : ExprPath = syn :: parse2 (quote ! { <# field_type as # cratename :: BorshSchema >:: add_definitions_recursively } ,) . unwrap () ; let path = definitions_override . unwrap_or (default_path) ; quote ! { # path (definitions) ; } }
};
}
