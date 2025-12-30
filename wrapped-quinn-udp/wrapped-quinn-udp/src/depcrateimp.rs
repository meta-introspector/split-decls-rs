// Generated macro for imp (module)
macro_rules! Depcrateimp {
() => {
// Module: crate
// Provides: {"imp"}
// Dependencies: {}
# [cfg (not (any (wasm_browser , unix , windows)))] # [path = "fallback.rs"] mod imp ;
};
}
