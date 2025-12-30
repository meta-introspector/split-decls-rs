// Generated macro for CanonicalInstantiator (struct)
macro_rules! Depcrate_infer_canonical_instantiateCanonicalInstantiator {
() => {
// Module: crate::infer::canonical::instantiate
// Provides: {"CanonicalInstantiator"}
// Dependencies: {}
# [doc = " Replaces the bound vars in a canonical binder with var values."] struct CanonicalInstantiator < 'tcx > { tcx : TyCtxt < 'tcx > , var_values : ty :: GenericArgsRef < 'tcx > , # [doc = " As with `BoundVarReplacer`, represents the index of a binder *just outside*"] # [doc = " the ones we have visited."] current_index : ty :: DebruijnIndex , cache : DelayedMap < (ty :: DebruijnIndex , Ty < 'tcx >) , Ty < 'tcx > > , }
};
}
