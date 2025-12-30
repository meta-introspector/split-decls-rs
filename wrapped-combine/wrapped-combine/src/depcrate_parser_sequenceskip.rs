// Generated macro for skip (function)
macro_rules! Depcrate_parser_sequenceskip {
() => {
// Module: crate::parser::sequence
// Provides: {"skip"}
// Dependencies: {}
pub fn skip < Input , P1 , P2 > (p1 : P1 , p2 : P2) -> Skip < P1 , P2 > where Input : Stream , P1 : Parser < Input > , P2 : Parser < Input > , { Skip ((p1 , ignore (p2))) }
};
}
