// Generated macro for vec_copy_to_u8 (function)
macro_rules! Depcrate_utilsvec_copy_to_u8 {
() => {
// Module: crate::utils
// Provides: {"vec_copy_to_u8"}
// Dependencies: {}
# [allow (dead_code)] pub (crate) fn vec_copy_to_u8 < T > (vec : & [T]) -> Vec < u8 > where T : bytemuck :: Pod , { bytemuck :: cast_slice (vec) . to_owned () }
};
}
