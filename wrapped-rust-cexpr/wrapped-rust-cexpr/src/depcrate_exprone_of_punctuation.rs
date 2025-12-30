// Generated macro for one_of_punctuation (function)
macro_rules! Depcrate_exprone_of_punctuation {
() => {
// Module: crate::expr
// Provides: {"one_of_punctuation"}
// Dependencies: {}
fn one_of_punctuation (c : & 'static [& 'static str]) -> impl Fn (& [Token]) -> CResult < '_ , & [u8] > { move | input | { if input . is_empty () { let min = c . iter () . map (| opt | opt . len ()) . min () . expect ("at least one option") ; Err (crate :: nom :: Err :: Incomplete (Needed :: new (min))) } else if input [0] . kind == TokenKind :: Punctuation && c . iter () . any (| opt | opt . as_bytes () == & input [0] . raw [..]) { Ok ((& input [1 ..] , & input [0] . raw [..])) } else { Err (crate :: nom :: Err :: Error ((input , crate :: ErrorKind :: ExactTokens (TokenKind :: Punctuation , c) ,) . into () ,)) } } }
};
}
