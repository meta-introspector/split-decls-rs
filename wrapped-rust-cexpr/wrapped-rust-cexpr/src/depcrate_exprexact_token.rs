// Generated macro for exact_token (macro)
macro_rules! Depcrate_exprexact_token {
() => {
// Module: crate::expr
// Provides: {"exact_token"}
// Dependencies: {}
macro_rules ! exact_token (($ k : ident , $ c : expr) => ({ move | input : & [Token] | { if input . is_empty () { let res : CResult <'_ , & [u8] > = Err (crate :: nom :: Err :: Incomplete (Needed :: new ($ c . len ()))) ; res } else { if input [0] . kind == TokenKind ::$ k && & input [0] . raw [..] ==$ c { Ok ((& input [1 ..] , & input [0] . raw [..])) } else { Err (crate :: nom :: Err :: Error ((input , crate :: ErrorKind :: ExactToken (TokenKind ::$ k ,$ c)) . into ())) } } } }) ;) ;
};
}
