// Generated macro for process_field (function)
macro_rules! Depcrate_internals_schema_structsprocess_field {
() => {
// Module: crate::internals::schema::structs
// Provides: {"process_field"}
// Dependencies: {}
fn process_field (field : & syn :: Field , cratename : & Path , fields_vec : & mut Vec < TokenStream2 > , add_definitions_recursively : & mut TokenStream2 ,) -> syn :: Result < () > { let parsed = field :: Attributes :: parse (& field . attrs) ? ; if ! parsed . skip { let field_name = field . ident . as_ref () ; let field_type = & field . ty ; fields_vec . push (field_declaration_output (field_name , field_type , cratename , parsed . schema_declaration () ,)) ; add_definitions_recursively . extend (field_definitions_output (field_type , cratename , parsed . schema_definitions () ,)) ; } Ok (()) }
};
}
