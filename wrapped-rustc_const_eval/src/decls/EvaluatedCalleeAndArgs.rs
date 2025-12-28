macro_rules! deps {
    () => {
        FnVal!();
        FnArg!();
        Machine!();
    };
}

macro_rules! EvaluatedCalleeAndArgs {
    () => {
        deps!();
        struct EvaluatedCalleeAndArgs < 'tcx , M : Machine < 'tcx > > { callee : FnVal < 'tcx , M :: ExtraFnVal > , args : Vec < FnArg < 'tcx , M :: Provenance > > , fn_sig : ty :: FnSig < 'tcx > , fn_abi : & 'tcx FnAbi < 'tcx , Ty < 'tcx > > , # [doc = " True if the function is marked as `#[track_caller]` ([`ty::InstanceKind::requires_caller_location`])"] with_caller_location : bool , }
    };
}

EvaluatedCalleeAndArgs!();