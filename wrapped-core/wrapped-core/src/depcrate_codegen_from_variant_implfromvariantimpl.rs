// Generated macro for FromVariantImpl (struct)
macro_rules! Depcrate_codegen_from_variant_implFromVariantImpl {
() => {
// Module: crate::codegen::from_variant_impl
// Provides: {"FromVariantImpl"}
// Dependencies: {}
pub struct FromVariantImpl < 'a > { pub base : TraitImpl < 'a > , # [doc = " If set, the ident of the field into which the variant ident should be placed."] # [doc = ""] # [doc = " This is one of `darling`'s \"magic fields\", which allow a type deriving a `darling`"] # [doc = " trait to get fields from the input `syn` element added to the deriving struct"] # [doc = " automatically."] pub ident : Option < & 'a Ident > , # [doc = " If set, the ident of the field into which the transformed output of the input"] # [doc = " variant's fields should be placed."] # [doc = ""] # [doc = " This is one of `darling`'s \"magic fields\"."] pub fields : Option < & 'a Ident > , # [doc = " If set, the ident of the field into which the discriminant of the input variant"] # [doc = " should be placed. The receiving field must be an `Option` as not all enums have"] # [doc = " discriminants."] # [doc = ""] # [doc = " This is one of `darling`'s \"magic fields\"."] pub discriminant : Option < & 'a Ident > , pub attr_names : & 'a PathList , pub forward_attrs : ForwardAttrs < 'a > , pub from_ident : bool , pub supports : Option < & 'a DataShape > , }
};
}
