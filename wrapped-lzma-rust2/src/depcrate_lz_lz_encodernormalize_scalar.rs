// Generated macro for normalize_scalar (function)
macro_rules! Depcrate_lz_lz_encodernormalize_scalar {
() => {
// Module: crate::lz::lz_encoder
// Provides: {"normalize_scalar"}
// Dependencies: {}
# [inline (always)] fn normalize_scalar (positions : & mut [i32] , norm_offset : i32) { positions . iter_mut () . for_each (| p | * p = p . saturating_sub (norm_offset)) ; }
};
}
