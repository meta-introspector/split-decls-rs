// Generated macro for and_then (function)
macro_rules! Depcrate_parser_combinatorand_then {
() => {
// Module: crate::parser::combinator
// Provides: {"and_then"}
// Dependencies: {}
# [doc = " Equivalent to [`p.and_then(f)`]."] # [doc = ""] # [doc = " [`p.and_then(f)`]: ../trait.Parser.html#method.and_then"] pub fn and_then < Input , P , F , O , E > (p : P , f : F) -> AndThen < P , F > where P : Parser < Input > , F : FnMut (P :: Output) -> Result < O , E > , Input : Stream , E : Into < < Input :: Error as ParseError < Input :: Token , Input :: Range , Input :: Position > > :: StreamError > , { AndThen (p , f) }
};
}
