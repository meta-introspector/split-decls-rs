// Generated macro for uncons (function)
macro_rules! Depcrate_streamuncons {
() => {
// Module: crate::stream
// Provides: {"uncons"}
// Dependencies: {}
# [inline] pub fn uncons < Input > (input : & mut Input) -> ParseResult < Input :: Token , Input :: Error > where Input : ? Sized + Stream , { match input . uncons () { Ok (x) => CommitOk (x) , Err (err) => wrap_stream_error (input , err) , } }
};
}
