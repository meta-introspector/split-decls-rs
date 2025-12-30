// Generated macro for ge_precompute (function)
macro_rules! Depcrate_edwards25519ge_precompute {
() => {
// Module: crate::edwards25519
// Provides: {"ge_precompute"}
// Dependencies: {}
fn ge_precompute (base : & GeP3) -> [GeCached ; 16] { let base_cached = base . to_cached () ; let mut pc = [GeP3 :: zero () ; 16] ; pc [1] = * base ; for i in 2 .. 16 { pc [i] = if i % 2 == 0 { pc [i / 2] . dbl () . to_p3 () } else { pc [i - 1] . add (base_cached) . to_p3 () } } let mut pc_cached : [GeCached ; 16] = Default :: default () ; for i in 0 .. 16 { pc_cached [i] = pc [i] . to_cached () ; } pc_cached }
};
}
