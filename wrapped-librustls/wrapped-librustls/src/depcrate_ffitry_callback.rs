// Generated macro for try_callback (macro)
macro_rules! Depcrate_ffitry_callback {
() => {
// Module: crate::ffi
// Provides: {"try_callback"}
// Dependencies: {}
macro_rules ! try_callback { ($ var : ident) => { match $ var { Some (c) => c , None => return $ crate :: panic :: NullParameterOrDefault :: value () , } } ; }
};
}
