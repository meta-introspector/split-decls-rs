// Generated macro for AVX (type)
macro_rules! Depcrate_x86_64AVX {
() => {
// Module: crate::x86_64
// Provides: {"AVX"}
// Dependencies: {}
# [doc = " AVX but not AVX2: only 128-bit integer operations, but use VEX versions of everything"] # [doc = " to avoid expensive SSE/VEX conflicts."] pub type AVX = SseMachine < YesS3 , YesS4 , NoNI > ;
};
}
