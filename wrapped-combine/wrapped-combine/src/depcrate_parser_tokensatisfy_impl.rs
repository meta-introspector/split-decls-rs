// Generated macro for satisfy_impl (function)
macro_rules! Depcrate_parser_tokensatisfy_impl {
() => {
// Module: crate::parser::token
// Provides: {"satisfy_impl"}
// Dependencies: {}
fn satisfy_impl < Input , P , R > (input : & mut Input , mut predicate : P) -> ParseResult < R , Input :: Error > where Input : Stream , P : FnMut (Input :: Token) -> Option < R > , { let position = input . position () ; match uncons (input) { PeekOk (c) | CommitOk (c) => match predicate (c) { Some (c) => CommitOk (c) , None => PeekErr (Input :: Error :: empty (position) . into ()) , } , PeekErr (err) => PeekErr (err) , CommitErr (err) => CommitErr (err) , } }
};
}
