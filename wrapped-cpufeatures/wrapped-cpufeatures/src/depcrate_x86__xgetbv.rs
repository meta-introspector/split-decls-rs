// Generated macro for __xgetbv (macro)
macro_rules! Depcrate_x86__xgetbv {
() => {
// Module: crate::x86
// Provides: {"__xgetbv"}
// Dependencies: {}
# [doc = " Check that OS supports required SIMD registers"] # [macro_export] # [doc (hidden)] macro_rules ! __xgetbv { ($ cr : expr , $ mask : expr) => { { # [cfg (target_arch = "x86")] use core :: arch :: x86 as arch ; # [cfg (target_arch = "x86_64")] use core :: arch :: x86_64 as arch ; let xmask = 0b11 << 26 ; let xsave = $ cr [0] . ecx & xmask == xmask ; if xsave { let xcr0 = unsafe { arch :: _xgetbv (arch :: _XCR_XFEATURE_ENABLED_MASK) } ; (xcr0 & $ mask) == $ mask } else { false } } } ; }
};
}
