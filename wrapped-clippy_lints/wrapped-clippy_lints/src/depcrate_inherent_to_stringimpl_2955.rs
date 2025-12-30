// Generated macro for impl_2955 (impl)
macro_rules! Depcrate_inherent_to_stringimpl_2955 {
() => {
// Module: crate::inherent_to_string
// Provides: {"impl_2955"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for InherentToString { fn check_impl_item (& mut self , cx : & LateContext < 'tcx > , impl_item : & 'tcx ImplItem < '_ >) { if let ImplItemKind :: Fn (ref signature , _) = impl_item . kind && let header = signature . header && header . is_safe () && header . abi == ExternAbi :: Rust && impl_item . ident . name == sym :: to_string && let decl = signature . decl && decl . implicit_self . has_implicit_self () && decl . inputs . len () == 1 && impl_item . generics . params . iter () . all (| p | matches ! (p . kind , GenericParamKind :: Lifetime { .. })) && ! impl_item . span . from_expansion () && is_type_lang_item (cx , return_ty (cx , impl_item . owner_id) , LangItem :: String) && trait_ref_of_method (cx , impl_item . owner_id) . is_none () { show_lint (cx , impl_item) ; } } }
};
}
