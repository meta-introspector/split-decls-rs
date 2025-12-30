// Generated macro for impl_26 (impl)
macro_rules! Depcrate_base_nimpl_26 {
() => {
// Module: crate::base_n
// Provides: {"impl_26"}
// Dependencies: {}
impl ToBaseN for u32 { fn encoded_len (base : usize) -> usize { let mut max = u32 :: MAX ; let mut len = 0 ; while max > 0 { len += 1 ; max /= base as u32 ; } len } }
};
}
