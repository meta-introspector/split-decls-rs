// Generated macro for probe_distance (function)
macro_rules! Depcrate_hpack_tableprobe_distance {
() => {
// Module: crate::hpack::table
// Provides: {"probe_distance"}
// Dependencies: {}
# [inline] fn probe_distance (mask : usize , hash : HashValue , current : usize) -> usize { current . wrapping_sub (desired_pos (mask , hash)) & mask }
};
}
