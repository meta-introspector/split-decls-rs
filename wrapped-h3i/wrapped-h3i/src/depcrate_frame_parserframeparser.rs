// Generated macro for FrameParser (struct)
macro_rules! Depcrate_frame_parserFrameParser {
() => {
// Module: crate::frame_parser
// Provides: {"FrameParser"}
// Dependencies: {}
# [doc = " Parses [`H3iFrame`]s from a QUIC stream."] # [doc = ""] # [doc = " Each `FrameParser` instance is bound to a single stream when created."] # [doc = " [`FrameParser::try_parse_frame()`] will attempt to pull stream data from a"] # [doc = " [`quiche::Connection`] and build a complete frame."] # [doc = ""] # [doc = " There are various success and failure criteria, see `try_parse_frame()` for"] # [doc = " specific guidance."] pub (crate) struct FrameParser { ty : Option < u64 > , len : Option < u64 > , stream_id : u64 , curr_state : FrameState , state_buf : Vec < u8 > , state_offset : usize , state_len : usize , }
};
}
