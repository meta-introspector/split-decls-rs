// Generated macro for abi (function)
macro_rules! Depcrate_grammarabi {
() => {
// Module: crate::grammar
// Provides: {"abi"}
// Dependencies: {}
fn abi (p : & mut Parser < '_ >) { assert ! (p . at (T ! [extern])) ; let abi = p . start () ; p . bump (T ! [extern]) ; p . eat (STRING) ; abi . complete (p , ABI) ; }
};
}
