// Generated macro for xcr0 (function)
macro_rules! Depcrate_controlregsxcr0 {
() => {
// Module: crate::controlregs
// Provides: {"xcr0"}
// Dependencies: {}
# [doc = " Read Extended Control Register XCR0."] # [doc = " Only supported if CR4_ENABLE_OS_XSAVE is set."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn xcr0 () -> Xcr0 { Xcr0 :: from_bits_truncate (_xgetbv (0)) }
};
}
