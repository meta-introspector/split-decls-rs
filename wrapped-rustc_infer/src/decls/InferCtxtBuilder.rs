macro_rules! InferCtxtBuilder {
    () => {
        # [doc = " Used to configure inference contexts before their creation."] pub struct InferCtxtBuilder < 'tcx > { tcx : TyCtxt < 'tcx > , considering_regions : bool , in_hir_typeck : bool , skip_leak_check : bool , # [doc = " Whether we should use the new trait solver in the local inference context,"] # [doc = " which affects things like which solver is used in `predicate_may_hold`."] next_trait_solver : bool , }
    };
}

InferCtxtBuilder!()