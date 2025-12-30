// Generated macro for QlogInfo (struct)
macro_rules! DepcrateQlogInfo {
() => {
// Module: crate
// Provides: {"QlogInfo"}
// Dependencies: {}
# [cfg (feature = "qlog")] struct QlogInfo { streamer : Option < qlog :: streamer :: QlogStreamer > , logged_peer_params : bool , level : EventImportance , }
};
}
