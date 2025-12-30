// Generated macro for RtlIsZeroLuid (function)
macro_rules! Depcrate_ntrtlRtlIsZeroLuid {
() => {
// Module: crate::ntrtl
// Provides: {"RtlIsZeroLuid"}
// Dependencies: {}
# [inline] pub const fn RtlIsZeroLuid (L1 : & LUID) -> bool { (L1 . LowPart | L1 . HighPart as u32) == 0 }
};
}
