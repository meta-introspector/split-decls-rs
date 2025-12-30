// Generated macro for ObligationInspector (type)
macro_rules! Depcrate_traitsObligationInspector {
() => {
// Module: crate::traits
// Provides: {"ObligationInspector"}
// Dependencies: {}
# [doc = " A callback that can be provided to `inspect_typeck`. Invoked on evaluation"] # [doc = " of root obligations."] pub type ObligationInspector < 'tcx > = fn (& InferCtxt < 'tcx > , & PredicateObligation < 'tcx > , Result < Certainty , NoSolution >) ;
};
}
