// Generated macro for identifier_token (function)
macro_rules! Depcrate_expridentifier_token {
() => {
// Module: crate::expr
// Provides: {"identifier_token"}
// Dependencies: {}
fn identifier_token (input : & [Token]) -> CResult < '_ , & [u8] > { if input . is_empty () { let res : CResult < '_ , & [u8] > = Err (nom :: Err :: Incomplete (Needed :: new (1))) ; res } else { if input [0] . kind == TokenKind :: Identifier { Ok ((& input [1 ..] , & input [0] . raw [..])) } else { Err (crate :: nom :: Err :: Error ((input , crate :: ErrorKind :: TypedToken (TokenKind :: Identifier)) . into ())) } } }
};
}
