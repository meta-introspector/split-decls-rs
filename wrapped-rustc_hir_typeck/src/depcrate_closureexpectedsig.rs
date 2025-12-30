// Generated macro for ExpectedSig (struct)
macro_rules! Depcrate_closureExpectedSig {
() => {
// Module: crate::closure
// Provides: {"ExpectedSig"}
// Dependencies: {}
# [doc = " What signature do we *expect* the closure to have from context?"] # [derive (Debug , Clone , TypeFoldable , TypeVisitable)] struct ExpectedSig < 'tcx > { # [doc = " Span that gave us this expectation, if we know that."] cause_span : Option < Span > , sig : ty :: PolyFnSig < 'tcx > , }
};
}
