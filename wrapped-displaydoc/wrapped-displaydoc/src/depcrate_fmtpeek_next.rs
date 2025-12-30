// Generated macro for peek_next (macro)
macro_rules! Depcrate_fmtpeek_next {
() => {
// Module: crate::fmt
// Provides: {"peek_next"}
// Dependencies: {}
macro_rules ! peek_next { ($ read : ident) => { match $ read . chars () . next () { Some (next) => next , None => return , } } ; }
};
}
