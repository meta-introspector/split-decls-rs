// Generated macro for impl_423 (impl)
macro_rules! Depcrate_infer_relate_type_relatingimpl_423 {
() => {
// Module: crate::infer::relate::type_relating
// Provides: {"impl_423"}
// Dependencies: {}
impl < 'infcx , 'tcx > TypeRelating < 'infcx , 'tcx > { pub (crate) fn new (infcx : & 'infcx InferCtxt < 'tcx > , trace : TypeTrace < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , define_opaque_types : DefineOpaqueTypes , ambient_variance : ty :: Variance ,) -> TypeRelating < 'infcx , 'tcx > { assert ! (! infcx . next_trait_solver) ; TypeRelating { infcx , trace , param_env , define_opaque_types , ambient_variance , obligations : PredicateObligations :: new () , cache : Default :: default () , } } pub (crate) fn into_obligations (self) -> PredicateObligations < 'tcx > { self . obligations } }
};
}
