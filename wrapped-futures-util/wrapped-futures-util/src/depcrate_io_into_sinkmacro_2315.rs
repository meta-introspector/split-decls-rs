// Generated macro for macro_2315 (macro)
macro_rules! Depcrate_io_into_sinkmacro_2315 {
() => {
// Module: crate::io::into_sink
// Provides: {"macro_2315"}
// Dependencies: {}
pin_project ! { # [doc = " Sink for the [`into_sink`](super::AsyncWriteExt::into_sink) method."] # [must_use = "sinks do nothing unless polled"] # [derive (Debug)] # [cfg_attr (docsrs , doc (cfg (feature = "sink")))] pub struct IntoSink < W , Item > { # [pin] writer : W , buffer : Option < Block < Item >>, } }
};
}
