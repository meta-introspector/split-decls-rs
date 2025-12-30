// Generated macro for Canonicalizer (struct)
macro_rules! Depcrate_infer_canonical_canonicalizerCanonicalizer {
() => {
// Module: crate::infer::canonical::canonicalizer
// Provides: {"Canonicalizer"}
// Dependencies: {}
struct Canonicalizer < 'cx , 'tcx > { # [doc = " Set to `None` to disable the resolution of inference variables."] infcx : Option < & 'cx InferCtxt < 'tcx > > , tcx : TyCtxt < 'tcx > , variables : SmallVec < [CanonicalVarKind < 'tcx > ; 8] > , query_state : & 'cx mut OriginalQueryValues < 'tcx > , indices : FxHashMap < GenericArg < 'tcx > , BoundVar > , # [doc = " Maps each `sub_unification_table_root_var` to the index of the first"] # [doc = " variable which used it."] # [doc = ""] # [doc = " This means in case two type variables have the same sub relations root,"] # [doc = " we set the `sub_root` of the second variable to the position of the first."] # [doc = " Otherwise the `sub_root` of each type variable is just its own position."] sub_root_lookup_table : SsoHashMap < ty :: TyVid , usize > , canonicalize_mode : & 'cx dyn CanonicalizeMode , needs_canonical_flags : TypeFlags , binder_index : ty :: DebruijnIndex , }
};
}
