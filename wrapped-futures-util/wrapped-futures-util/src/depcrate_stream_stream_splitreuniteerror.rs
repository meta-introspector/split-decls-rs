// Generated macro for ReuniteError (struct)
macro_rules! Depcrate_stream_stream_splitReuniteError {
() => {
// Module: crate::stream::stream::split
// Provides: {"ReuniteError"}
// Dependencies: {}
# [doc = " Error indicating a `SplitSink<S>` and `SplitStream<S>` were not two halves"] # [doc = " of a `Stream + Split`, and thus could not be `reunite`d."] # [cfg_attr (docsrs , doc (cfg (feature = "sink")))] pub struct ReuniteError < T , Item > (pub SplitSink < T , Item > , pub SplitStream < T >) ;
};
}
