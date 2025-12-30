// Generated macro for check_copy_clone (function)
macro_rules! Depcrate_derivecheck_copy_clone {
() => {
// Module: crate::derive
// Provides: {"check_copy_clone"}
// Dependencies: {}
# [doc = " Implementation of the `EXPL_IMPL_CLONE_ON_COPY` lint."] fn check_copy_clone < 'tcx > (cx : & LateContext < 'tcx > , item : & Item < '_ > , trait_ref : & hir :: TraitRef < '_ > , ty : Ty < 'tcx >) { let clone_id = match cx . tcx . lang_items () . clone_trait () { Some (id) if trait_ref . trait_def_id () == Some (id) => id , _ => return , } ; let Some (copy_id) = cx . tcx . lang_items () . copy_trait () else { return ; } ; let (ty_adt , ty_subs) = match * ty . kind () { ty :: Adt (adt , subs) if ! adt . is_union () => (adt , subs) , _ => return , } ; if ! is_copy (cx , ty) { if ty_subs . non_erasable_generics () . next () . is_some () { let has_copy_impl = cx . tcx . local_trait_impls (copy_id) . iter () . any (| & id | { matches ! (cx . tcx . type_of (id) . instantiate_identity () . kind () , ty :: Adt (adt , _) if ty_adt . did () == adt . did ()) }) ; if ! has_copy_impl { return ; } } else { return ; } } if ty_subs . types () . any (| ty | ! implements_trait (cx , ty , clone_id , & [])) { return ; } if ty_adt . repr () . packed () && ty_subs . iter () . any (| arg | matches ! (arg . kind () , GenericArgKind :: Type (_) | GenericArgKind :: Const (_))) { return ; } if ty_adt . all_fields () . any (| f | f . safety . is_unsafe ()) { return ; } span_lint_and_note (cx , EXPL_IMPL_CLONE_ON_COPY , item . span , "you are implementing `Clone` explicitly on a `Copy` type" , Some (item . span) , "consider deriving `Clone` or removing `Copy`" ,) ; }
};
}
