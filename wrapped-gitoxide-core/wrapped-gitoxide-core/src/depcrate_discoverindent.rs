// Generated macro for indent (function)
macro_rules! Depcrate_discoverindent {
() => {
// Module: crate::discover
// Provides: {"indent"}
// Dependencies: {}
fn indent (mut out : impl std :: io :: Write , msg : impl Into < String >) -> std :: io :: Result < () > { for line in msg . into () . lines () { writeln ! (out , "\t{line}") ? ; } Ok (()) }
};
}
