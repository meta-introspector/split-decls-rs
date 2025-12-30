// Generated macro for impl_1816 (impl)
macro_rules! Depcrate_vec_is_zeroimpl_1816 {
() => {
// Module: crate::vec::is_zero
// Provides: {"impl_1816"}
// Dependencies: {}
unsafe impl < T : IsZero , const N : usize > IsZero for [T ; N] { # [inline] fn is_zero (& self) -> bool { N <= 16 && self . iter () . all (IsZero :: is_zero) } }
};
}
