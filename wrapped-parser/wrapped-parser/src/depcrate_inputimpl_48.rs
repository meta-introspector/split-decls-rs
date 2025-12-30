// Generated macro for impl_48 (impl)
macro_rules! Depcrate_inputimpl_48 {
() => {
// Module: crate::input
// Provides: {"impl_48"}
// Dependencies: {}
impl Input { fn bit_index (& self , n : usize) -> (usize , usize) { let idx = n / (bits :: BITS as usize) ; let b_idx = n % (bits :: BITS as usize) ; (idx , b_idx) } fn len (& self) -> usize { self . kind . len () } }
};
}
