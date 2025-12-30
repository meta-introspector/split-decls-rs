// Generated macro for probe_distance (function)
macro_rules! Depcrate_header_mapprobe_distance {
() => {
// Module: crate::header::map
// Provides: {"probe_distance"}
// Dependencies: {}
# [doc = " The number of steps that `current` is forward of the desired position for hash"] # [inline] fn probe_distance (mask : Size , hash : HashValue , current : usize) -> usize { current . wrapping_sub (desired_pos (mask , hash)) & mask as usize }
};
}
