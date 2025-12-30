// Generated macro for ALIGN_MASK (const)
macro_rules! Depcrate_asciiALIGN_MASK {
() => {
// Module: crate::ascii
// Provides: {"ALIGN_MASK"}
// Dependencies: {}
# [cfg (any (test , miri , not (target_arch = "x86_64")))] const ALIGN_MASK : usize = core :: mem :: align_of :: < usize > () - 1 ;
};
}
