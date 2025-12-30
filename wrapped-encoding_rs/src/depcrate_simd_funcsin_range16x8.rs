// Generated macro for in_range16x8 (macro)
macro_rules! Depcrate_simd_funcsin_range16x8 {
() => {
// Module: crate::simd_funcs
// Provides: {"in_range16x8"}
// Dependencies: {}
macro_rules ! in_range16x8 { ($ s : ident , $ start : expr , $ end : expr) => { { ($ s - u16x8 :: splat ($ start)) . simd_lt (u16x8 :: splat ($ end - $ start)) } } ; }
};
}
