// Generated macro for parse_fn (function)
macro_rules! Depcrate_parseparse_fn {
() => {
// Module: crate::parse
// Provides: {"parse_fn"}
// Dependencies: {}
fn parse_fn < T > (input : ParseStream) -> parse :: Result < Expr > where T : Parse , { input . parse :: < Token ! [fn] > () ? ; input . parse :: < T > () ? ; input . parse :: < Token ! [=] > () ? ; let path = input . parse () ? ; input . parse :: < Token ! [;] > () ? ; Ok (path) }
};
}
