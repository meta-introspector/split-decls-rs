// Generated macro for Error (enum)
macro_rules! Depcrate_stream_decoderError {
() => {
// Module: crate::stream::decoder
// Provides: {"Error"}
// Dependencies: {}
# [derive (Debug)] pub enum Error < E , P > { Parse (E) , Io { position : P , error : io :: Error } , }
};
}
