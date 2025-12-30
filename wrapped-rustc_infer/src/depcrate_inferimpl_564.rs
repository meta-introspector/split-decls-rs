// Generated macro for impl_564 (impl)
macro_rules! Depcrate_inferimpl_564 {
() => {
// Module: crate::infer
// Provides: {"impl_564"}
// Dependencies: {}
impl < 'tcx , T > InferOk < 'tcx , T > { # [doc = " Extracts `value`, registering any obligations into `fulfill_cx`."] pub fn into_value_registering_obligations < E : 'tcx > (self , infcx : & InferCtxt < 'tcx > , fulfill_cx : & mut dyn TraitEngine < 'tcx , E > ,) -> T { let InferOk { value , obligations } = self ; fulfill_cx . register_predicate_obligations (infcx , obligations) ; value } }
};
}
