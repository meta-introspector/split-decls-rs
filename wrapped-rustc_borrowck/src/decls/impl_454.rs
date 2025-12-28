macro_rules! deps {
    () => {
        LivenessContext!();
    };
}

macro_rules! impl_454 {
    () => {
        deps!();
        impl < 'a , 'typeck , 'tcx > LivenessContext < 'a , 'typeck , 'tcx > { # [doc = " Computes the `MaybeInitializedPlaces` dataflow analysis if it hasn't been done already."] # [doc = ""] # [doc = " In practice, the results of this dataflow analysis are rarely needed but can be expensive to"] # [doc = " compute on big functions, so we compute them lazily as a fast path when:"] # [doc = " - there are relevant live locals"] # [doc = " - there are drop points for these relevant live locals."] # [doc = ""] # [doc = " This happens as part of the drop-liveness computation: it's the only place checking for"] # [doc = " maybe-initializedness of `MovePathIndex`es."] fn flow_inits (& mut self) -> & mut ResultsCursor < 'a , 'tcx , MaybeInitializedPlaces < 'a , 'tcx > > { self . flow_inits . get_or_insert_with (| | { let tcx = self . typeck . tcx () ; let body = self . typeck . body ; let flow_inits = MaybeInitializedPlaces :: new (tcx , body , self . move_data) . iterate_to_fixpoint (tcx , body , Some ("borrowck")) . into_results_cursor (body) ; flow_inits }) } }
    };
}

impl_454!()