// Generated macro for impl_3372 (impl)
macro_rules! Depcrate_lifetimesimpl_3372 {
() => {
// Module: crate::lifetimes
// Provides: {"impl_3372"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for RefVisitor < '_ , 'tcx > { fn visit_lifetime (& mut self , lifetime : & 'tcx Lifetime) { self . lts . push (* lifetime) ; } fn visit_poly_trait_ref (& mut self , poly_tref : & 'tcx PolyTraitRef < 'tcx >) { let trait_ref = & poly_tref . trait_ref ; if let Some (id) = trait_ref . trait_def_id () && lang_items :: FN_TRAITS . iter () . any (| & item | self . cx . tcx . lang_items () . get (item) == Some (id)) { let mut sub_visitor = RefVisitor :: new (self . cx) ; sub_visitor . visit_trait_ref (trait_ref) ; self . nested_elision_site_lts . append (& mut sub_visitor . all_lts ()) ; } else { walk_poly_trait_ref (self , poly_tref) ; } } fn visit_ty (& mut self , ty : & 'tcx Ty < '_ , AmbigArg >) { match ty . kind { TyKind :: FnPtr (& FnPtrTy { decl , .. }) => { let mut sub_visitor = RefVisitor :: new (self . cx) ; sub_visitor . visit_fn_decl (decl) ; self . nested_elision_site_lts . append (& mut sub_visitor . all_lts ()) ; } , TyKind :: TraitObject (bounds , lt) => { if ! lt . is_elided () { self . unelided_trait_object_lifetime = true ; } for bound in bounds { self . visit_poly_trait_ref (bound) ; } } , _ => walk_ty (self , ty) , } } }
};
}
