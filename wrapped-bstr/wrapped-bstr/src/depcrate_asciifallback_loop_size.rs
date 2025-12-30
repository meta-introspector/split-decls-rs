// Generated macro for FALLBACK_LOOP_SIZE (const)
macro_rules! Depcrate_asciiFALLBACK_LOOP_SIZE {
() => {
// Module: crate::ascii
// Provides: {"FALLBACK_LOOP_SIZE"}
// Dependencies: {}
# [cfg (any (test , miri , not (target_arch = "x86_64")))] const FALLBACK_LOOP_SIZE : usize = 2 * USIZE_BYTES ;
};
}
