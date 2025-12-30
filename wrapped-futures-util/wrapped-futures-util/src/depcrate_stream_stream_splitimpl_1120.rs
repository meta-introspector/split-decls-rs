// Generated macro for impl_1120 (impl)
macro_rules! Depcrate_stream_stream_splitimpl_1120 {
() => {
// Module: crate::stream::stream::split
// Provides: {"impl_1120"}
// Dependencies: {}
impl < S : Unpin > SplitStream < S > { # [doc = " Attempts to put the two \"halves\" of a split `Stream + Sink` back"] # [doc = " together. Succeeds only if the `SplitStream<S>` and `SplitSink<S>` are"] # [doc = " a matching pair originating from the same call to `StreamExt::split`."] pub fn reunite < Item > (self , other : SplitSink < S , Item >) -> Result < S , ReuniteError < S , Item > > where S : Sink < Item > , { other . reunite (self) } }
};
}
