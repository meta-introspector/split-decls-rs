// Generated macro for try_take (macro)
macro_rules! Depcrate_ffitry_take {
() => {
// Module: crate::ffi
// Provides: {"try_take"}
// Dependencies: {}
macro_rules ! try_take { ($ var : ident) => { match $ var . take () { None => { return $ crate :: rustls_result :: AlreadyUsed ; } Some (x) => x , } } ; }
};
}
