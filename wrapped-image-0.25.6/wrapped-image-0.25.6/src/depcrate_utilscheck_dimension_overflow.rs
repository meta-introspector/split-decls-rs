// Generated macro for check_dimension_overflow (function)
macro_rules! Depcrate_utilscheck_dimension_overflow {
() => {
// Module: crate::utils
// Provides: {"check_dimension_overflow"}
// Dependencies: {}
# [doc = " Checks if the provided dimensions would cause an overflow."] # [allow (dead_code)] pub (crate) fn check_dimension_overflow (width : u32 , height : u32 , bytes_per_pixel : u8) -> bool { u64 :: from (width) * u64 :: from (height) > u64 :: MAX / u64 :: from (bytes_per_pixel) }
};
}
