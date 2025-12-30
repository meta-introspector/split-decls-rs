// Generated macro for process_variant (function)
macro_rules! Depcrate_internals_schema_enumsprocess_variant {
() => {
// Module: crate::internals::schema::enums
// Provides: {"process_variant"}
// Dependencies: {}
fn process_variant (variant : & Variant , discriminant_info : DiscriminantInfo , cratename : & Path , enum_name : & str , enum_generics : & Generics , generics_output : & mut schema :: GenericsOutput ,) -> syn :: Result < VariantOutput > { let variant_name = variant . ident . to_token_stream () . to_string () ; let full_variant_name = format ! ("{}{}" , enum_name , variant_name) ; let full_variant_ident = Ident :: new (& full_variant_name , Span :: call_site ()) ; schema :: visit_struct_fields (& variant . fields , & mut generics_output . params_visitor) ? ; let (inner_struct , inner_struct_generics) = inner_struct_definition (variant , cratename , & full_variant_ident , enum_generics) ; let (_ig , inner_struct_ty_generics , _wc) = inner_struct_generics . split_for_impl () ; let variant_type = quote ! { <# full_variant_ident # inner_struct_ty_generics as # cratename :: BorshSchema > } ; let discriminant_value = process_discriminant (& variant . ident , discriminant_info) ? ; Ok (VariantOutput { inner_struct , add_definitions_recursively_call : quote ! { # variant_type :: add_definitions_recursively (definitions) ; } , variant_entry : quote ! { (u8 :: from (# discriminant_value) as i64 , # variant_name . into () , # variant_type :: declaration ()) } , }) }
};
}
