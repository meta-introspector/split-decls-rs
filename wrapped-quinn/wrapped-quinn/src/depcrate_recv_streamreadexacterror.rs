// Generated macro for ReadExactError (enum)
macro_rules! Depcrate_recv_streamReadExactError {
() => {
// Module: crate::recv_stream
// Provides: {"ReadExactError"}
// Dependencies: {}
# [doc = " Errors that arise from reading from a stream."] # [derive (Debug , Error , Clone , PartialEq , Eq)] pub enum ReadExactError { # [doc = " The stream finished before all bytes were read"] # [error ("stream finished early ({0} bytes read)")] FinishedEarly (usize) , # [doc = " A read error occurred"] # [error (transparent)] ReadError (# [from] ReadError) , }
};
}
