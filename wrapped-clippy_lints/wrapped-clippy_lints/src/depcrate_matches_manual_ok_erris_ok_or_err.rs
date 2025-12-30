// Generated macro for is_ok_or_err (function)
macro_rules! Depcrate_matches_manual_ok_erris_ok_or_err {
() => {
// Module: crate::matches::manual_ok_err
// Provides: {"is_ok_or_err"}
// Dependencies: {}
# [doc = " Return `Some((true, IDENT))` if `pat` contains `Ok(IDENT)`, `Some((false, IDENT))` if it"] # [doc = " contains `Err(IDENT)`, `None` otherwise."] fn is_ok_or_err < 'hir > (cx : & LateContext < '_ > , pat : & Pat < 'hir >) -> Option < (bool , & 'hir Ident) > { if let PatKind :: TupleStruct (qpath , [arg] , _) = & pat . kind && let PatKind :: Binding (BindingMode :: NONE , _ , ident , None) = & arg . kind && let res = cx . qpath_res (qpath , pat . hir_id) && let Res :: Def (DefKind :: Ctor (..) , id) = res && let id @ Some (_) = cx . tcx . opt_parent (id) { let lang_items = cx . tcx . lang_items () ; if id == lang_items . result_ok_variant () { return Some ((true , ident)) ; } else if id == lang_items . result_err_variant () { return Some ((false , ident)) ; } } None }
};
}
