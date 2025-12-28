macro_rules! ExpectedSig {
    () => {
        # [doc = " What signature do we *expect* the closure to have from context?"] # [derive (Debug , Clone , TypeFoldable , TypeVisitable)] struct ExpectedSig < 'tcx > { # [doc = " Span that gave us this expectation, if we know that."] cause_span : Option < Span > , sig : ty :: PolyFnSig < 'tcx > , }
    };
}

ExpectedSig!()