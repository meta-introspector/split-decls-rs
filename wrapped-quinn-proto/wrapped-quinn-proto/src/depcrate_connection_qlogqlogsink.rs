// Generated macro for QlogSink (struct)
macro_rules! Depcrate_connection_qlogQlogSink {
() => {
// Module: crate::connection::qlog
// Provides: {"QlogSink"}
// Dependencies: {}
# [doc = " A [`QlogStream`] that may be either dynamically disabled or compiled out entirely"] # [derive (Clone , Default)] pub (crate) struct QlogSink { # [cfg (feature = "qlog")] stream : Option < QlogStream > , }
};
}
