// Generated macro for bit_set (function)
macro_rules! Depcrate_samplingbit_set {
() => {
// Module: crate::sampling
// Provides: {"bit_set"}
// Dependencies: {}
fn bit_set (z : & [u8] , i : usize) -> bool { let bit_index = i & 0x07 ; let byte_index = i >> 3 ; z [byte_index] & (1 << bit_index) != 0 }
};
}
