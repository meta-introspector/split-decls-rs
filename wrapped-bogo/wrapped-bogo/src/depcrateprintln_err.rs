// Generated macro for println_err (macro)
macro_rules! Depcrateprintln_err {
() => {
// Module: crate
// Provides: {"println_err"}
// Dependencies: {}
macro_rules ! println_err (($ ($ arg : tt) *) => { { writeln ! (& mut :: std :: io :: stderr () , $ ($ arg) *) . unwrap () ; } }) ;
};
}
