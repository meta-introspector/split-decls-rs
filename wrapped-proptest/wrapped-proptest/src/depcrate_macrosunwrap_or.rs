// Generated macro for unwrap_or (macro)
macro_rules! Depcrate_macrosunwrap_or {
() => {
// Module: crate::macros
// Provides: {"unwrap_or"}
// Dependencies: {}
macro_rules ! unwrap_or { ($ unwrap : expr , $ err : ident => $ on_err : expr) => { match $ unwrap { Ok (ok) => ok , Err ($ err) => $ on_err , } } ; }
};
}
