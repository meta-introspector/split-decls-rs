// Generated macro for impl_7140 (impl)
macro_rules! Depcrate_min_ident_charsimpl_7140 {
() => {
// Module: crate::min_ident_chars
// Provides: {"impl_7140"}
// Dependencies: {}
impl LateLintPass < '_ > for MinIdentChars { fn check_item (& mut self , cx : & LateContext < '_ > , item : & Item < '_ >) { if self . min_ident_chars_threshold == 0 { return ; } walk_item (& mut IdentVisitor { conf : self , cx } , item) ; } fn check_trait_item (& mut self , cx : & LateContext < '_ > , item : & TraitItem < '_ >) { if self . min_ident_chars_threshold == 0 { return ; } if matches ! (& item . kind , rustc_hir :: TraitItemKind :: Fn (_ , _)) { let param_names = cx . tcx . fn_arg_idents (item . owner_id . to_def_id ()) ; for ident in param_names . iter () . flatten () { let str = ident . as_str () ; if self . is_ident_too_short (cx , str , ident . span) { emit_min_ident_chars (self , cx , str , ident . span) ; } } } walk_trait_item (& mut IdentVisitor { conf : self , cx } , item) ; } fn check_pat (& mut self , cx : & LateContext < '_ > , pat : & Pat < '_ >) { if let PatKind :: Binding (_ , _ , ident , ..) = pat . kind && let str = ident . as_str () && self . is_ident_too_short (cx , str , ident . span) && is_not_in_trait_impl (cx , pat , ident) { emit_min_ident_chars (self , cx , str , ident . span) ; } } }
};
}
