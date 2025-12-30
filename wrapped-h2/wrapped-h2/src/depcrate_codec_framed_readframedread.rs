// Generated macro for FramedRead (struct)
macro_rules! Depcrate_codec_framed_readFramedRead {
() => {
// Module: crate::codec::framed_read
// Provides: {"FramedRead"}
// Dependencies: {}
# [derive (Debug)] pub struct FramedRead < T > { inner : InnerFramedRead < T , LengthDelimitedCodec > , hpack : hpack :: Decoder , max_header_list_size : usize , max_continuation_frames : usize , partial : Option < Partial > , }
};
}
