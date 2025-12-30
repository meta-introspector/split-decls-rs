// Generated macro for hash (function)
macro_rules! Depcrate_parking_lothash {
() => {
// Module: crate::parking_lot
// Provides: {"hash"}
// Dependencies: {}
# [cfg (target_pointer_width = "64")] # [inline] fn hash (key : usize , bits : u32) -> usize { key . wrapping_mul (0x9E3779B97F4A7C15) >> (64 - bits) }
};
}
