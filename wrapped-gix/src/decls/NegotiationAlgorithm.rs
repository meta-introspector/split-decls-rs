macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! NegotiationAlgorithm {
    () => {
        deps!();
        # [doc = " The `fetch.negotiationAlgorithm` key."] pub type NegotiationAlgorithm = keys :: Any < validate :: NegotiationAlgorithm > ;
    };
}

NegotiationAlgorithm!()