// Generated macro for SplitSink (struct)
macro_rules! Depcrate_stream_stream_splitSplitSink {
() => {
// Module: crate::stream::stream::split
// Provides: {"SplitSink"}
// Dependencies: {}
# [doc = " A `Sink` part of the split pair"] # [derive (Debug)] # [must_use = "sinks do nothing unless polled"] # [cfg_attr (docsrs , doc (cfg (feature = "sink")))] pub struct SplitSink < S , Item > { lock : BiLock < S > , slot : Option < Item > , }
};
}
