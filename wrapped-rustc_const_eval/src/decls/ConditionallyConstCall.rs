macro_rules! ConditionallyConstCall {
    () => {
        # [doc = " A call to a function that is in a trait, or has trait bounds that make it conditionally-const."] # [derive (Debug)] pub (crate) struct ConditionallyConstCall < 'tcx > { pub callee : DefId , pub args : GenericArgsRef < 'tcx > , pub span : Span , pub call_source : CallSource , }
    };
}

ConditionallyConstCall!();