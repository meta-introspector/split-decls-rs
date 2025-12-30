// Generated macro for parse (function)
macro_rules! Depcrateparse {
() => {
// Module: crate
// Provides: {"parse"}
// Dependencies: {}
fn parse (re : & str) -> Result < Expr > { ExprBuilder :: new () . allow_bytes (true) . parse (re) . map_err (From :: from) }
};
}
