// Generated macro for arg_error (macro)
macro_rules! Depcratearg_error {
() => {
// Module: crate
// Provides: {"arg_error"}
// Dependencies: {}
macro_rules ! arg_error { ($ ($ err : tt) *) => { { eprintln ! ($ ($ err) *) ; eprintln ! () ; usage () ; std :: process :: exit (1) ; } } ; }
};
}
