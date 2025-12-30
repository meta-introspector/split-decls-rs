// Generated macro for ok (macro)
macro_rules! Depcrate_macrosok {
() => {
// Module: crate::macros
// Provides: {"ok"}
// Dependencies: {}
macro_rules ! ok { ($ expr : expr) => { match $ expr { Ok (val) => val , Err (err) => { return Err (err) ; } } } ; }
};
}
