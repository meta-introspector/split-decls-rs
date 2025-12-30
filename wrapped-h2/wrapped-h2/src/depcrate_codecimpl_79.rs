// Generated macro for impl_79 (impl)
macro_rules! Depcrate_codecimpl_79 {
() => {
// Module: crate::codec
// Provides: {"impl_79"}
// Dependencies: {}
impl < T , B > Codec < T , B > where T : AsyncRead + AsyncWrite + Unpin , B : Buf , { # [doc = " Returns a new `Codec` with the default max frame size"] # [inline] pub fn new (io : T) -> Self { Self :: with_max_recv_frame_size (io , frame :: DEFAULT_MAX_FRAME_SIZE as usize) } # [doc = " Returns a new `Codec` with the given maximum frame size"] pub fn with_max_recv_frame_size (io : T , max_frame_size : usize) -> Self { let framed_write = FramedWrite :: new (io) ; let delimited = length_delimited :: Builder :: new () . big_endian () . length_field_length (3) . length_adjustment (9) . num_skip (0) . new_read (framed_write) ; let mut inner = FramedRead :: new (delimited) ; inner . set_max_frame_size (max_frame_size) ; Codec { inner } } }
};
}
