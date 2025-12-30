// Generated macro for impl_562 (impl)
macro_rules! Depcrate_inferimpl_562 {
() => {
// Module: crate::infer
// Provides: {"impl_562"}
// Dependencies: {}
# [extension (pub trait TyCtxtInferExt <'tcx >)] impl < 'tcx > TyCtxt < 'tcx > { fn infer_ctxt (self) -> InferCtxtBuilder < 'tcx > { InferCtxtBuilder { tcx : self , considering_regions : true , in_hir_typeck : false , skip_leak_check : false , next_trait_solver : self . next_trait_solver_globally () , } } }
};
}
