// Generated macro for complete (macro)
macro_rules! Depcrate_macroscomplete {
() => {
// Module: crate::macros
// Provides: {"complete"}
// Dependencies: {}
macro_rules ! complete { ($ e : expr) => { match $ e ? { Status :: Complete (v) => v , Status :: Partial => return Ok (Status :: Partial) } } }
};
}
