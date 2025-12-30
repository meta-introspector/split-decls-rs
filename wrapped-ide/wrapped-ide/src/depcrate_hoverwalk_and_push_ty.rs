// Generated macro for walk_and_push_ty (function)
macro_rules! Depcrate_hoverwalk_and_push_ty {
() => {
// Module: crate::hover
// Provides: {"walk_and_push_ty"}
// Dependencies: {}
fn walk_and_push_ty (db : & RootDatabase , ty : & hir :: Type < '_ > , push_new_def : & mut dyn FnMut (hir :: ModuleDef) ,) { ty . walk (db , | t | { if let Some (adt) = t . as_adt () { push_new_def (adt . into ()) ; } else if let Some (trait_) = t . as_dyn_trait () { push_new_def (trait_ . into ()) ; } else if let Some (traits) = t . as_impl_traits (db) { traits . for_each (| it | push_new_def (it . into ())) ; } else if let Some (trait_) = t . as_associated_type_parent_trait (db) { push_new_def (trait_ . into ()) ; } else if let Some (tp) = t . as_type_param (db) { let sized_trait = LangItem :: Sized . resolve_trait (db , t . krate (db) . into ()) ; tp . trait_bounds (db) . into_iter () . filter (| & it | Some (it . into ()) != sized_trait) . for_each (| it | push_new_def (it . into ())) ; } }) ; }
};
}
