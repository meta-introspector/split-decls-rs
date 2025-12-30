// Generated macro for impl_464 (impl)
macro_rules! Depcrate_impl_trait_overcapturesimpl_464 {
() => {
// Module: crate::impl_trait_overcaptures
// Provides: {"impl_464"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for ImplTraitOvercaptures { fn check_item (& mut self , cx : & LateContext < 'tcx > , it : & 'tcx hir :: Item < 'tcx >) { match & it . kind { hir :: ItemKind :: Fn { .. } => check_fn (cx . tcx , it . owner_id . def_id) , _ => { } } } fn check_impl_item (& mut self , cx : & LateContext < 'tcx > , it : & 'tcx hir :: ImplItem < 'tcx >) { match & it . kind { hir :: ImplItemKind :: Fn (_ , _) => check_fn (cx . tcx , it . owner_id . def_id) , _ => { } } } fn check_trait_item (& mut self , cx : & LateContext < 'tcx > , it : & 'tcx hir :: TraitItem < 'tcx >) { match & it . kind { hir :: TraitItemKind :: Fn (_ , _) => check_fn (cx . tcx , it . owner_id . def_id) , _ => { } } } }
};
}
