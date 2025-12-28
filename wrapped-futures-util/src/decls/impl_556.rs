macro_rules! deps {
    () => {
        SplitSink!();
        Sink!();
        SplitStream!();
        ReuniteError!();
    };
}

macro_rules! impl_556 {
    () => {
        deps!();
        impl < S : Unpin > SplitStream < S > { # [doc = " Attempts to put the two \"halves\" of a split `Stream + Sink` back"] # [doc = " together. Succeeds only if the `SplitStream<S>` and `SplitSink<S>` are"] # [doc = " a matching pair originating from the same call to `StreamExt::split`."] pub fn reunite < Item > (self , other : SplitSink < S , Item >) -> Result < S , ReuniteError < S , Item > > where S : Sink < Item > , { other . reunite (self) } }
    };
}

impl_556!();