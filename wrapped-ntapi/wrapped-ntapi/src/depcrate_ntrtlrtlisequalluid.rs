// Generated macro for RtlIsEqualLuid (function)
macro_rules! Depcrate_ntrtlRtlIsEqualLuid {
() => {
// Module: crate::ntrtl
// Provides: {"RtlIsEqualLuid"}
// Dependencies: {}
# [inline] pub const fn RtlIsEqualLuid (L1 : & LUID , L2 : & LUID) -> bool { (L1 . LowPart == L2 . LowPart) && (L1 . HighPart == L2 . HighPart) }
};
}
