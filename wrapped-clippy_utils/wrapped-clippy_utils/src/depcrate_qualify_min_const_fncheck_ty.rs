// Generated macro for check_ty (function)
macro_rules! Depcrate_qualify_min_const_fncheck_ty {
() => {
// Module: crate::qualify_min_const_fn
// Provides: {"check_ty"}
// Dependencies: {}
fn check_ty < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx > , span : Span , msrv : Msrv) -> McfResult { for arg in ty . walk () { let ty = match arg . kind () { GenericArgKind :: Type (ty) => ty , GenericArgKind :: Lifetime (_) | GenericArgKind :: Const (_) => continue , } ; match ty . kind () { ty :: Ref (_ , _ , hir :: Mutability :: Mut) if ! msrv . meets (cx , msrvs :: CONST_MUT_REFS) => { return Err ((span , "mutable references in const fn are unstable" . into ())) ; } , ty :: Alias (ty :: Opaque , ..) => return Err ((span , "`impl Trait` in const fn is unstable" . into ())) , ty :: FnPtr (..) => { return Err ((span , "function pointers in const fn are unstable" . into ())) ; } , ty :: Dynamic (preds , _) => { for pred in * preds { match pred . skip_binder () { ty :: ExistentialPredicate :: AutoTrait (_) | ty :: ExistentialPredicate :: Projection (_) => { return Err ((span , "trait bounds other than `Sized` \
                                 on const fn parameters are unstable" . into () ,)) ; } , ty :: ExistentialPredicate :: Trait (trait_ref) => { if Some (trait_ref . def_id) != cx . tcx . lang_items () . sized_trait () { return Err ((span , "trait bounds other than `Sized` \
                                     on const fn parameters are unstable" . into () ,)) ; } } , } } } , _ => { } , } } Ok (()) }
};
}
