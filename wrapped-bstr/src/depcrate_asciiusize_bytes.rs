// Generated macro for USIZE_BYTES (const)
macro_rules! Depcrate_asciiUSIZE_BYTES {
() => {
// Module: crate::ascii
// Provides: {"USIZE_BYTES"}
// Dependencies: {}
# [cfg (any (test , miri , not (target_arch = "x86_64")))] const USIZE_BYTES : usize = core :: mem :: size_of :: < usize > () ;
};
}
