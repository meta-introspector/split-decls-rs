// Generated macro for impl_457 (impl)
macro_rules! Depcrate_parser_combinatorimpl_457 {
() => {
// Module: crate::parser::combinator
// Provides: {"impl_457"}
// Dependencies: {}
impl < F , P > Recognize < F , P > { # [inline] fn recognize_result < Input > (elements : & mut F , before : < Input as ResetStream > :: Checkpoint , input : & mut Input , result : ParseResult < P :: Output , < Input as StreamOnce > :: Error > ,) -> ParseResult < F , < Input as StreamOnce > :: Error > where P : Parser < Input > , Input : Stream , F : Default + Extend < Input :: Token > , { match result { PeekOk (_) => { let last_position = input . position () ; ctry ! (input . reset (before) . committed ()) ; while input . position () != last_position { match input . uncons () { Ok (elem) => elements . extend (Some (elem)) , Err (err) => { return PeekErr (< Input as StreamOnce > :: Error :: from_error (input . position () , err) . into () ,) ; } } } PeekOk (mem :: take (elements)) } CommitOk (_) => { let last_position = input . position () ; ctry ! (input . reset (before) . committed ()) ; while input . position () != last_position { match input . uncons () { Ok (elem) => elements . extend (Some (elem)) , Err (err) => { return CommitErr (< Input as StreamOnce > :: Error :: from_error (input . position () , err ,)) ; } } } CommitOk (mem :: take (elements)) } CommitErr (err) => { let last_position = input . position () ; ctry ! (input . reset (before) . committed ()) ; while input . position () != last_position { match input . uncons () { Ok (elem) => elements . extend (Some (elem)) , Err (err) => { return CommitErr (< Input as StreamOnce > :: Error :: from_error (input . position () , err ,)) ; } } } CommitErr (err) } PeekErr (err) => PeekErr (err) , } } }
};
}
