// Generated macro for expected_token (macro)
macro_rules! Depcrateexpected_token {
() => {
// Module: crate
// Provides: {"expected_token"}
// Dependencies: {}
macro_rules ! expected_token { ($ sig : tt .$ item : tt () , $ msg : expr) => { if let None = $ sig .$ item () { bail ! ($ sig , "expected {}" , $ msg) ; } } ; }
};
}
