// Generated macro for mix_columns (function)
macro_rules! Depcrate_utilsmix_columns {
() => {
// Module: crate::utils
// Provides: {"mix_columns"}
// Dependencies: {}
# [allow (clippy :: needless_range_loop)] pub (crate) fn mix_columns < const N : usize > (state : & mut [u64 ; N]) { state . iter_mut () . for_each (mix_u64) ; }
};
}
