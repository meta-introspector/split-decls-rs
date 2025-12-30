// Generated macro for format_variances (function)
macro_rules! Depcrate_variance_dumpformat_variances {
() => {
// Module: crate::variance::dump
// Provides: {"format_variances"}
// Dependencies: {}
fn format_variances (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> String { let variances = tcx . variances_of (def_id) ; let generics = GenericArgs :: identity_for_item (tcx , def_id) ; let mut ret = String :: with_capacity (2 + 7 * variances . len ()) ; ret . push ('[') ; for (arg , variance) in generics . iter () . zip (variances . iter ()) { write ! (ret , "{arg}: {variance:?}, ") . unwrap () ; } if ! variances . is_empty () { ret . pop () ; ret . pop () ; } ret . push (']') ; ret }
};
}
