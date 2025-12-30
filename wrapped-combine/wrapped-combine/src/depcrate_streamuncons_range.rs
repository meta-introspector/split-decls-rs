// Generated macro for uncons_range (function)
macro_rules! Depcrate_streamuncons_range {
() => {
// Module: crate::stream
// Provides: {"uncons_range"}
// Dependencies: {}
# [inline] pub fn uncons_range < Input > (input : & mut Input , size : usize ,) -> ParseResult < Input :: Range , < Input as StreamOnce > :: Error > where Input : ? Sized + RangeStream , { match input . uncons_range (size) { Err (err) => wrap_stream_error (input , err) , Ok (x) => { if size == 0 { PeekOk (x) } else { CommitOk (x) } } } }
};
}
