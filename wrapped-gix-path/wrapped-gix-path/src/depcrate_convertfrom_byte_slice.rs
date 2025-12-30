// Generated macro for from_byte_slice (function)
macro_rules! Depcrate_convertfrom_byte_slice {
() => {
// Module: crate::convert
// Provides: {"from_byte_slice"}
// Dependencies: {}
# [doc = " Similar to [`try_from_byte_slice()`], but will **panic** if there is ill-formed UTF-8 in the `input`."] pub fn from_byte_slice (input : & [u8]) -> & Path { try_from_byte_slice (input) . expect ("well-formed UTF-8 on windows") }
};
}
