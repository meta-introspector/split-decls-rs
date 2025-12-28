macro_rules! NGHTTP2_ERR_FLOODED {
    () => {
        # [doc = " Possible flooding by peer was detected in this HTTP/2 session."] # [doc = " Flooding is measured by how many PING and SETTINGS frames with"] # [doc = " ACK flag set are queued for transmission.  These frames are"] # [doc = " response for the peer initiated frames, and peer can cause memory"] # [doc = " exhaustion on server side to send these frames forever and does"] # [doc = " not read network."] pub const NGHTTP2_ERR_FLOODED : nghttp2_error = - 904 ;
    };
}

NGHTTP2_ERR_FLOODED!()