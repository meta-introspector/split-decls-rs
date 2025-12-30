// Generated macro for lower_variant_ctor (function)
macro_rules! Depcrate_collectlower_variant_ctor {
() => {
// Module: crate::collect
// Provides: {"lower_variant_ctor"}
// Dependencies: {}
pub (super) fn lower_variant_ctor (tcx : TyCtxt < '_ > , def_id : LocalDefId) { tcx . ensure_ok () . generics_of (def_id) ; tcx . ensure_ok () . type_of (def_id) ; tcx . ensure_ok () . predicates_of (def_id) ; }
};
}
