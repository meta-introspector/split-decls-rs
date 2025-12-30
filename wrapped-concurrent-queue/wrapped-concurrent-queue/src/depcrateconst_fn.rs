// Generated macro for const_fn (macro)
macro_rules! Depcrateconst_fn {
() => {
// Module: crate
// Provides: {"const_fn"}
// Dependencies: {}
# [doc = " Make the given function const if the given condition is true."] macro_rules ! const_fn { (const_if : # [cfg ($ ($ cfg : tt) +)] ; $ (# [$ ($ attr : tt) *]) * $ vis : vis const fn $ ($ rest : tt) *) => { # [cfg ($ ($ cfg) +)] $ (# [$ ($ attr) *]) * $ vis const fn $ ($ rest) * # [cfg (not ($ ($ cfg) +))] $ (# [$ ($ attr) *]) * $ vis fn $ ($ rest) * } ; }
};
}
