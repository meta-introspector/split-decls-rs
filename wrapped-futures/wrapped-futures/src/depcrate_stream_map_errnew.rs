// Generated macro for new (function)
macro_rules! Depcrate_stream_map_errnew {
() => {
// Module: crate::stream::map_err
// Provides: {"new"}
// Dependencies: {}
pub fn new < S , F , U > (s : S , f : F) -> MapErr < S , F > where S : Stream , F : FnMut (S :: Error) -> U + Send + 'static , U : Send + 'static , { MapErr { stream : s , f : f , } }
};
}
