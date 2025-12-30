// Generated macro for Variant (struct)
macro_rules! Depcrate_codegen_variantVariant {
() => {
// Module: crate::codegen::variant
// Provides: {"Variant"}
// Dependencies: {}
# [doc = " A variant of the enum which is deriving `FromMeta`."] # [derive (Debug , Clone)] pub struct Variant < 'a > { # [doc = " The name which will appear in code passed to the `FromMeta` input."] pub name_in_attr : Cow < 'a , str > , # [doc = " The name of the variant which will be returned for a given `name_in_attr`."] pub variant_ident : & 'a Ident , # [doc = " The name of the parent enum type."] pub ty_ident : & 'a Ident , pub data : Fields < Field < 'a > > , # [doc = " Whether or not the variant should be skipped in the generated code."] pub skip : bool , pub allow_unknown_fields : bool , }
};
}
