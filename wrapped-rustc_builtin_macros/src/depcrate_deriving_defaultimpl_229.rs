// Generated macro for impl_229 (impl)
macro_rules! Depcrate_deriving_defaultimpl_229 {
() => {
// Module: crate::deriving::default
// Provides: {"impl_229"}
// Dependencies: {}
impl < 'a , 'b > rustc_ast :: visit :: Visitor < 'a > for DetectNonVariantDefaultAttr < 'a , 'b > { fn visit_attribute (& mut self , attr : & 'a rustc_ast :: Attribute) { if attr . has_name (kw :: Default) { let post = if self . cx . ecfg . features . default_field_values () { " or variants where every field has a default value" } else { "" } ; self . cx . dcx () . emit_err (errors :: NonUnitDefault { span : attr . span , post }) ; } rustc_ast :: visit :: walk_attribute (self , attr) ; } fn visit_variant (& mut self , v : & 'a rustc_ast :: Variant) { self . visit_ident (& v . ident) ; self . visit_vis (& v . vis) ; self . visit_variant_data (& v . data) ; visit_opt ! (self , visit_anon_const , & v . disr_expr) ; for attr in & v . attrs { rustc_ast :: visit :: walk_attribute (self , attr) ; } } }
};
}
