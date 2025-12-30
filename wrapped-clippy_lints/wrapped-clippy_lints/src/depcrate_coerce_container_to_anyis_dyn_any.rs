// Generated macro for is_dyn_any (function)
macro_rules! Depcrate_coerce_container_to_anyis_dyn_any {
() => {
// Module: crate::coerce_container_to_any
// Provides: {"is_dyn_any"}
// Dependencies: {}
fn is_dyn_any (tcx : TyCtxt < '_ > , ty : Ty < '_ >) -> bool { let ty :: Dynamic (traits , ..) = ty . kind () else { return false ; } ; traits . iter () . any (| binder | { let ExistentialPredicate :: Trait (t) = binder . skip_binder () else { return false ; } ; tcx . is_diagnostic_item (sym :: Any , t . def_id) }) }
};
}
