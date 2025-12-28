macro_rules! deps {
    () => {
        InferCtxtBuilder!();
    };
}

macro_rules! impl_267 {
    () => {
        deps!();
        # [extension (pub trait TyCtxtInferExt <'tcx >)] impl < 'tcx > TyCtxt < 'tcx > { fn infer_ctxt (self) -> InferCtxtBuilder < 'tcx > { InferCtxtBuilder { tcx : self , considering_regions : true , in_hir_typeck : false , skip_leak_check : false , next_trait_solver : self . next_trait_solver_globally () , } } }
    };
}

impl_267!()