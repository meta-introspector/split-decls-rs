// Generated macro for uncons_while (function)
macro_rules! Depcrate_streamuncons_while {
() => {
// Module: crate::stream
// Provides: {"uncons_while"}
// Dependencies: {}
# [doc = " Removes items from the input while `predicate` returns `true`."] # [inline] pub fn uncons_while < Input , F > (input : & mut Input , predicate : F ,) -> ParseResult < Input :: Range , Input :: Error > where F : FnMut (Input :: Token) -> bool , Input : ? Sized + RangeStream , Input :: Range : Range , { match input . uncons_while (predicate) { Err (err) => wrap_stream_error (input , err) , Ok (x) => { if input . is_partial () && input_at_eof (input) { CommitErr (Input :: Error :: from_error (input . position () , StreamError :: end_of_input () ,)) } else if x . len () == 0 { PeekOk (x) } else { CommitOk (x) } } } }
};
}
