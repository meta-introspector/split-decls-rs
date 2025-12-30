// Generated macro for hex_check_neon (function)
macro_rules! Depcrate_decodehex_check_neon {
() => {
// Module: crate::decode
// Provides: {"hex_check_neon"}
// Dependencies: {}
# [target_feature (enable = "neon")] # [cfg (target_arch = "aarch64")] pub unsafe fn hex_check_neon (src : & [u8]) -> bool { hex_check_neon_with_case (src , CheckCase :: None) }
};
}
