// Generated macro for Encoder (struct)
macro_rules! Depcrate_codec_framed_writeEncoder {
() => {
// Module: crate::codec::framed_write
// Provides: {"Encoder"}
// Dependencies: {}
# [derive (Debug)] struct Encoder < B > { # [doc = " HPACK encoder"] hpack : hpack :: Encoder , # [doc = " Write buffer"] # [doc = ""] # [doc = " TODO: Should this be a ring buffer?"] buf : Cursor < BytesMut > , # [doc = " Next frame to encode"] next : Option < Next < B > > , # [doc = " Last data frame"] last_data_frame : Option < frame :: Data < B > > , # [doc = " Max frame size, this is specified by the peer"] max_frame_size : FrameSize , # [doc = " Chain payloads bigger than this."] chain_threshold : usize , # [doc = " Min buffer required to attempt to write a frame"] min_buffer_capacity : usize , }
};
}
