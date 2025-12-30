// Generated macro for add_constant_xor (function)
macro_rules! Depcrate_utilsadd_constant_xor {
() => {
// Module: crate::utils
// Provides: {"add_constant_xor"}
// Dependencies: {}
pub (crate) fn add_constant_xor < const N : usize > (state : & mut [u64 ; N] , round : usize) { for (i , word) in state . iter_mut () . enumerate () { let constant = ((i * 0x10) ^ round) as u64 ; * word ^= constant << 56 ; } }
};
}
