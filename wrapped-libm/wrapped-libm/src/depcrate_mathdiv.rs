// Generated macro for div (macro)
macro_rules! Depcrate_mathdiv {
() => {
// Module: crate::math
// Provides: {"div"}
// Dependencies: {}
# [cfg (all (not (debug_assertions) , intrinsics_enabled))] macro_rules ! div { ($ a : expr , $ b : expr) => { unsafe { core :: intrinsics :: unchecked_div ($ a , $ b) } } ; }
};
}
