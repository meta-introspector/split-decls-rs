// Generated macro for is_ty_or_ty_ctxt (function)
macro_rules! Depcrate_internalis_ty_or_ty_ctxt {
() => {
// Module: crate::internal
// Provides: {"is_ty_or_ty_ctxt"}
// Dependencies: {}
fn is_ty_or_ty_ctxt (cx : & LateContext < '_ > , path : & hir :: Path < '_ >) -> Option < String > { match & path . res { Res :: Def (_ , def_id) => { if let Some (name @ (sym :: Ty | sym :: TyCtxt)) = cx . tcx . get_diagnostic_name (* def_id) { return Some (format ! ("{}{}" , name , gen_args (path . segments . last () . unwrap ()))) ; } } Res :: SelfTyAlias { alias_to : did , is_trait_impl : false , .. } => { if let ty :: Adt (adt , args) = cx . tcx . type_of (did) . instantiate_identity () . kind () && let Some (name @ (sym :: Ty | sym :: TyCtxt)) = cx . tcx . get_diagnostic_name (adt . did ()) { return Some (format ! ("{}<{}>" , name , args [0])) ; } } _ => () , } None }
};
}
