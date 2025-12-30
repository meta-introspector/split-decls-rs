// Generated macro for impl_288 (impl)
macro_rules! Depcrate_voidimpl_288 {
() => {
// Module: crate::void
// Provides: {"impl_288"}
// Dependencies: {}
impl < 'ast > visit :: Visit < 'ast > for Uninhabited { fn visit_type_never (& mut self , _ : & 'ast syn :: TypeNever) { self . set () ; } fn visit_type_path (& mut self , type_path : & 'ast syn :: TypePath) { const KNOWN_UNINHABITED : & [& str] = & ["std::string::ParseError" , "::std::string::ParseError"] ; if type_path . qself . is_none () && util :: match_pathsegs (& type_path . path , KNOWN_UNINHABITED) { self . set () ; } } fn visit_type_array (& mut self , arr : & 'ast syn :: TypeArray) { if let Some (len) = interp :: eval_expr (& arr . len) { if len > 0 { self . visit_type (& arr . elem) ; } } } fn visit_type_bare_fn (& mut self , _ : & 'ast syn :: TypeBareFn) { } fn visit_macro (& mut self , _ : & 'ast syn :: Macro) { } fn visit_type_impl_trait (& mut self , _ : & 'ast syn :: TypeImplTrait) { } fn visit_type_trait_object (& mut self , _ : & 'ast syn :: TypeTraitObject) { } }
};
}
