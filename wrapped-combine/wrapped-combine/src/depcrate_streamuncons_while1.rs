// Generated macro for uncons_while1 (function)
macro_rules! Depcrate_streamuncons_while1 {
() => {
// Module: crate::stream
// Provides: {"uncons_while1"}
// Dependencies: {}
# [inline] # [doc = " Takes items from stream, testing each one with `predicate`"] # [doc = " returns a range of at least one items which passed `predicate`."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This may not return `PeekOk` as it should uncons at least one token."] pub fn uncons_while1 < Input , F > (input : & mut Input , predicate : F ,) -> ParseResult < Input :: Range , Input :: Error > where F : FnMut (Input :: Token) -> bool , Input : ? Sized + RangeStream , { match input . uncons_while1 (predicate) { CommitOk (x) => { if input . is_partial () && input_at_eof (input) { CommitErr (Input :: Error :: from_error (input . position () , StreamError :: end_of_input () ,)) } else { CommitOk (x) } } PeekErr (_) => { if input . is_partial () && input_at_eof (input) { CommitErr (Input :: Error :: from_error (input . position () , StreamError :: end_of_input () ,)) } else { PeekErr (Input :: Error :: empty (input . position ()) . into ()) } } CommitErr (err) => { if input . is_partial () && input_at_eof (input) { CommitErr (Input :: Error :: from_error (input . position () , StreamError :: end_of_input () ,)) } else { wrap_stream_error (input , err) } } PeekOk (_) => unreachable ! () , } }
};
}
