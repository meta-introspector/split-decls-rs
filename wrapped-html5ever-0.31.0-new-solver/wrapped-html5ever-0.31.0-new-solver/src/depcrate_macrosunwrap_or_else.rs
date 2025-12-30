// Generated macro for unwrap_or_else (macro)
macro_rules! Depcrate_macrosunwrap_or_else {
() => {
// Module: crate::macros
// Provides: {"unwrap_or_else"}
// Dependencies: {}
macro_rules ! unwrap_or_else { ($ opt : expr , $ else_block : block) => { { let Some (x) = $ opt else { $ else_block } ; x } } ; }
};
}
