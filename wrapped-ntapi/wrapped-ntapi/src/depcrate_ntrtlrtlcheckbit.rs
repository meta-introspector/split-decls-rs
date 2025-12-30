// Generated macro for RtlCheckBit (function)
macro_rules! Depcrate_ntrtlRtlCheckBit {
() => {
// Module: crate::ntrtl
// Provides: {"RtlCheckBit"}
// Dependencies: {}
# [inline] pub unsafe fn RtlCheckBit (BitMapHeader : & RTL_BITMAP , BitPosition : ULONG) -> u8 { # [cfg (target_arch = "x86_64")] { core :: arch :: x86_64 :: _bittest64 (BitMapHeader . Buffer as * const i64 , BitPosition as i64) } # [cfg (any (target_arch = "x86" , target_arch = "aarch64"))] { (* BitMapHeader . Buffer . offset (BitPosition as isize / 32) >> (BitPosition % 32) & 1) as u8 } }
};
}
