// Generated macro for RtlConvertLongToLuid (function)
macro_rules! Depcrate_ntrtlRtlConvertLongToLuid {
() => {
// Module: crate::ntrtl
// Provides: {"RtlConvertLongToLuid"}
// Dependencies: {}
# [inline] pub const fn RtlConvertLongToLuid (Long : LONG) -> LUID { LUID { LowPart : Long as u32 , HighPart : ((Long as i64) >> 32) as i32 } }
};
}
