// Generated macro for next (macro)
macro_rules! Depcrate_macrosnext {
() => {
// Module: crate::macros
// Provides: {"next"}
// Dependencies: {}
macro_rules ! next { ($ bytes : ident) => ({ match $ bytes . next () { Some (b) => b , None => return Ok (Status :: Partial) } }) }
};
}
