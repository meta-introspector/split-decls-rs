macro_rules! deps {
    () => {
        SplitStream!();
        ReuniteError!();
        Sink!();
        SplitSink!();
    };
}

macro_rules! impl_561 {
    () => {
        deps!();
        impl < S : Sink < Item > + Unpin , Item > SplitSink < S , Item > { # [doc = " Attempts to put the two \"halves\" of a split `Stream + Sink` back"] # [doc = " together. Succeeds only if the `SplitStream<S>` and `SplitSink<S>` are"] # [doc = " a matching pair originating from the same call to `StreamExt::split`."] pub fn reunite (self , other : SplitStream < S >) -> Result < S , ReuniteError < S , Item > > { self . lock . reunite (other . 0) . map_err (| err | ReuniteError (SplitSink (err . 0) , SplitStream (err . 1))) } }
    };
}

impl_561!();