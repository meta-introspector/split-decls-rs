// Generated macro for add_constant_plus (function)
macro_rules! Depcrate_utilsadd_constant_plus {
() => {
// Module: crate::utils
// Provides: {"add_constant_plus"}
// Dependencies: {}
pub (crate) fn add_constant_plus < const N : usize > (state : & mut [u64 ; N] , round : usize) { for (i , word) in state . iter_mut () . enumerate () { * word = word . swap_bytes () . wrapping_add (0x00F0F0F0F0F0F0F3u64 ^ (((((N - i - 1) * 0x10) ^ round) as u64) << 56)) . swap_bytes () ; } }
};
}
