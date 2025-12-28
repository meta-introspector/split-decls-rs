macro_rules! GateReason {
    () => {
        enum GateReason { Experimental , ImplDetail , }
    };
}

GateReason!()