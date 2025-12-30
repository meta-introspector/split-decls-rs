// Generated macro for FrameParseResult (enum)
macro_rules! Depcrate_frame_parserFrameParseResult {
() => {
// Module: crate::frame_parser
// Provides: {"FrameParseResult"}
// Dependencies: {}
# [derive (Debug , Eq , PartialEq)] # [doc = " Represents different frame parsing outcomes."] pub enum FrameParseResult { # [doc = " The frame was unable to be parsed at the current moment. This signifies"] # [doc = " that the stream is retryable without another I/O cycle. If another"] # [doc = " I/O cycle is needed, a [`quiche::h3::Error::TransportError`]"] # [doc = " containing [`quiche::Error::Done`] will be returned."] Retry , # [doc = " A frame has been successfully parsed. `fin` denotes if the FIN bit was"] # [doc = " set."] FrameParsed { h3i_frame : H3iFrame , fin : bool } , # [doc = " A frame is in the middle of being parsed, but either a FIN bit or a"] # [doc = " RESET_STREAM was received."] Interrupted (InterruptCause) , }
};
}
