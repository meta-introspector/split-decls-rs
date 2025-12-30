// Generated macro for process_fields (function)
macro_rules! Depcrate_internals_schema_structsprocess_fields {
() => {
// Module: crate::internals::schema::structs
// Provides: {"process_fields"}
// Dependencies: {}
fn process_fields (cratename : & Path , fields : & Fields , generics : & mut schema :: GenericsOutput ,) -> syn :: Result < (TokenStream2 , TokenStream2) > { let mut struct_fields = TokenStream2 :: new () ; let mut add_definitions_recursively = TokenStream2 :: new () ; let mut fields_vec = vec ! [] ; schema :: visit_struct_fields (fields , & mut generics . params_visitor) ? ; match fields { Fields :: Named (fields) => { for field in & fields . named { process_field (field , cratename , & mut fields_vec , & mut add_definitions_recursively ,) ? ; } if ! fields_vec . is_empty () { struct_fields = quote ! { let fields = # cratename :: schema :: Fields :: NamedFields (# cratename :: __private :: maybestd :: vec ! [# (# fields_vec) ,*]) ; } ; } } Fields :: Unnamed (fields) => { for field in & fields . unnamed { process_field (field , cratename , & mut fields_vec , & mut add_definitions_recursively ,) ? ; } if ! fields_vec . is_empty () { struct_fields = quote ! { let fields = # cratename :: schema :: Fields :: UnnamedFields (# cratename :: __private :: maybestd :: vec ! [# (# fields_vec) ,*]) ; } ; } } Fields :: Unit => { } } if fields_vec . is_empty () { struct_fields = quote ! { let fields = # cratename :: schema :: Fields :: Empty ; } ; } Ok ((struct_fields , add_definitions_recursively)) }
};
}
