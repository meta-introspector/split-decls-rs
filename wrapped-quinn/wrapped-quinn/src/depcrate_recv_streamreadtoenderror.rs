// Generated macro for ReadToEndError (enum)
macro_rules! Depcrate_recv_streamReadToEndError {
() => {
// Module: crate::recv_stream
// Provides: {"ReadToEndError"}
// Dependencies: {}
# [doc = " Errors from [`RecvStream::read_to_end`]"] # [derive (Debug , Error , Clone , PartialEq , Eq)] pub enum ReadToEndError { # [doc = " An error occurred during reading"] # [error ("read error: {0}")] Read (# [from] ReadError) , # [doc = " The stream is larger than the user-supplied limit"] # [error ("stream too long")] TooLong , }
};
}
