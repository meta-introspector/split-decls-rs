// Generated macro for impl_25 (impl)
macro_rules! Depcrate_base_nimpl_25 {
() => {
// Module: crate::base_n
// Provides: {"impl_25"}
// Dependencies: {}
impl ToBaseN for u64 { fn encoded_len (base : usize) -> usize { let mut max = u64 :: MAX ; let mut len = 0 ; while max > 0 { len += 1 ; max /= base as u64 ; } len } }
};
}
