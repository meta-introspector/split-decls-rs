macro_rules! deps {
    () => {
        PredicateObligation!();
        InferCtxt!();
    };
}

macro_rules! ObligationInspector {
    () => {
        deps!();
        # [doc = " A callback that can be provided to `inspect_typeck`. Invoked on evaluation"] # [doc = " of root obligations."] pub type ObligationInspector < 'tcx > = fn (& InferCtxt < 'tcx > , & PredicateObligation < 'tcx > , Result < Certainty , NoSolution >) ;
    };
}

ObligationInspector!();