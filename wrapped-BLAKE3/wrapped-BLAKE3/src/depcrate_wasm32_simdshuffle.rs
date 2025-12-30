// Generated macro for shuffle (macro)
macro_rules! Depcrate_wasm32_simdshuffle {
() => {
// Module: crate::wasm32_simd
// Provides: {"shuffle"}
// Dependencies: {}
macro_rules ! shuffle { ($ a : expr , $ b : expr , $ z : expr , $ y : expr , $ x : expr , $ w : expr) => { i32x4_shuffle ::< { $ w } , { $ x } , { $ y + 4 } , { $ z + 4 } > ($ a , $ b) } ; }
};
}
