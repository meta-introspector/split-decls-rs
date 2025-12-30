// Generated macro for const_fn (macro)
macro_rules! Depcrate_utilsconst_fn {
() => {
// Module: crate::utils
// Provides: {"const_fn"}
// Dependencies: {}
# [doc = " Make the given function const if the given condition is true."] macro_rules ! const_fn { (const_if : # [cfg ($ ($ cfg : tt) +)] ; $ (# [$ ($ attr : tt) *]) * $ vis : vis const $ ($ rest : tt) *) => { # [cfg ($ ($ cfg) +)] $ (# [$ ($ attr) *]) * $ vis const $ ($ rest) * # [cfg (not ($ ($ cfg) +))] $ (# [$ ($ attr) *]) * $ vis $ ($ rest) * } ; }
};
}
