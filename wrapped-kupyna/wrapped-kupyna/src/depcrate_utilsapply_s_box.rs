// Generated macro for apply_s_box (function)
macro_rules! Depcrate_utilsapply_s_box {
() => {
// Module: crate::utils
// Provides: {"apply_s_box"}
// Dependencies: {}
pub (crate) fn apply_s_box < const N : usize > (state : & mut [u64 ; N]) { for word in state . iter_mut () { let bytes = word . to_be_bytes () ; let transformed_bytes = array :: from_fn (| i | SBOXES [i % 4] [bytes [i] as usize]) ; * word = u64 :: from_be_bytes (transformed_bytes) ; } }
};
}
