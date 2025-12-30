// Generated macro for QlogStream (struct)
macro_rules! Depcrate_connection_qlogQlogStream {
() => {
// Module: crate::connection::qlog
// Provides: {"QlogStream"}
// Dependencies: {}
# [doc = " Shareable handle to a single qlog output stream"] # [cfg (feature = "qlog")] # [derive (Clone)] pub struct QlogStream (pub (crate) Arc < Mutex < QlogStreamer > >) ;
};
}
