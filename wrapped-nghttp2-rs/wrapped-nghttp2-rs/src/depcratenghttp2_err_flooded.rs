// Generated macro for NGHTTP2_ERR_FLOODED (const)
macro_rules! DepcrateNGHTTP2_ERR_FLOODED {
() => {
// Module: crate
// Provides: {"NGHTTP2_ERR_FLOODED"}
// Dependencies: {}
# [doc = " Possible flooding by peer was detected in this HTTP/2 session."] # [doc = " Flooding is measured by how many PING and SETTINGS frames with"] # [doc = " ACK flag set are queued for transmission.  These frames are"] # [doc = " response for the peer initiated frames, and peer can cause memory"] # [doc = " exhaustion on server side to send these frames forever and does"] # [doc = " not read network."] pub const NGHTTP2_ERR_FLOODED : nghttp2_error = - 904 ;
};
}
