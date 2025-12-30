// Generated macro for process_variant (function)
macro_rules! Depcrate_internals_serialize_enumsprocess_variant {
() => {
// Module: crate::internals::serialize::enums
// Provides: {"process_variant"}
// Dependencies: {}
fn process_variant (variant : & Variant , enum_ident : & Ident , discriminant_value : & TokenStream2 , cratename : & Path , generics : & mut serialize :: GenericsOutput ,) -> syn :: Result < VariantOutput > { let variant_ident = & variant . ident ; let variant_output = match & variant . fields { Fields :: Named (fields) => { let mut variant_fields = VariantFields :: default () ; for field in & fields . named { let field_id = serialize :: FieldId :: Enum (field . ident . clone () . unwrap ()) ; process_field (field , field_id , cratename , generics , & mut variant_fields) ? ; } VariantOutput { body : VariantBody :: Fields (variant_fields . named_header ()) , variant_idx_body : quote ! (# enum_ident ::# variant_ident { .. } => # discriminant_value ,) , } } Fields :: Unnamed (fields) => { let mut variant_fields = VariantFields :: default () ; for (field_idx , field) in fields . unnamed . iter () . enumerate () { let field_id = serialize :: FieldId :: new_enum_unnamed (field_idx) ? ; process_field (field , field_id , cratename , generics , & mut variant_fields) ? ; } VariantOutput { body : VariantBody :: Fields (variant_fields . unnamed_header ()) , variant_idx_body : quote ! (# enum_ident ::# variant_ident (..) => # discriminant_value ,) , } } Fields :: Unit => VariantOutput { body : VariantBody :: Unit , variant_idx_body : quote ! (# enum_ident ::# variant_ident => # discriminant_value ,) , } , } ; Ok (variant_output) }
};
}
