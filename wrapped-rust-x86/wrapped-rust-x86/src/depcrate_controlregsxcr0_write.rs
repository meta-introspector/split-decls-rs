// Generated macro for xcr0_write (function)
macro_rules! Depcrate_controlregsxcr0_write {
() => {
// Module: crate::controlregs
// Provides: {"xcr0_write"}
// Dependencies: {}
# [doc = " Write to Extended Control Register XCR0."] # [doc = " Only supported if CR4_ENABLE_OS_XSAVE is set."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn xcr0_write (val : Xcr0) { _xsetbv (0 , val . bits) ; }
};
}
