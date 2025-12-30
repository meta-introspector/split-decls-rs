// Generated macro for impl_511 (impl)
macro_rules! Depcrate_options_coreimpl_511 {
() => {
// Module: crate::options::core
// Provides: {"impl_511"}
// Dependencies: {}
impl < 'a > From < & 'a Core > for codegen :: TraitImpl < 'a > { fn from (v : & 'a Core) -> Self { codegen :: TraitImpl { ident : & v . ident , generics : & v . generics , data : v . data . as_ref () . map_struct_fields (InputField :: as_codegen_field) . map_enum_variants (| variant | variant . as_codegen_variant (& v . ident)) , default : v . as_codegen_default () , post_transform : v . post_transform . as_ref () , allow_unknown_fields : v . allow_unknown_fields . unwrap_or_default () , } } }
};
}
