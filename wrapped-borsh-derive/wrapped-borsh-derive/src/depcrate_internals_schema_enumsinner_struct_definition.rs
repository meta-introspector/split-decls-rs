// Generated macro for inner_struct_definition (function)
macro_rules! Depcrate_internals_schema_enumsinner_struct_definition {
() => {
// Module: crate::internals::schema::enums
// Provides: {"inner_struct_definition"}
// Dependencies: {}
fn inner_struct_definition (variant : & Variant , cratename : & Path , inner_struct_ident : & Ident , enum_generics : & Generics ,) -> (TokenStream2 , Generics) { let transformed_fields = transform_variant_fields (variant . fields . clone ()) ; let mut variant_schema_params_visitor = generics :: FindTyParams :: new (enum_generics) ; schema :: visit_struct_fields_unconditional (& variant . fields , & mut variant_schema_params_visitor) ; let variant_not_skipped_params = variant_schema_params_visitor . process_for_params () . into_iter () . collect :: < HashSet < _ > > () ; let inner_struct_generics = schema :: filter_used_params (enum_generics , variant_not_skipped_params) ; let inner_struct = ItemStruct { attrs : vec ! [] , vis : Visibility :: Inherited , struct_token : Default :: default () , ident : inner_struct_ident . clone () , generics : inner_struct_generics . clone () , fields : transformed_fields , semi_token : Some (Default :: default ()) , } ; let crate_str = syn :: LitStr :: new (& cratename . to_token_stream () . to_string () , Span :: call_site ()) ; let inner_struct = quote ! { # [allow (dead_code)] # [derive (# cratename :: BorshSchema)] # [borsh (crate = # crate_str)] # inner_struct } ; (inner_struct , inner_struct_generics) }
};
}
