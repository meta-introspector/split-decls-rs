// Generated macro for swapgs (function)
macro_rules! Depcrate_bits64_segmentationswapgs {
() => {
// Module: crate::bits64::segmentation
// Provides: {"swapgs"}
// Dependencies: {}
# [doc = " Swap the GS register."] # [doc = ""] # [doc = " Exchanges the current GS base register value with the value contained"] # [doc = " in MSR address IA32_KERNEL_GS_BASE."] # [doc = ""] # [doc = " The SWAPGS instruction is available only in 64-bit mode."] # [doc = ""] # [doc = " # Safety"] # [doc = " The SWAPGS instruction is a privileged instruction intended for use by system software."] # [cfg (target_arch = "x86_64")] pub unsafe fn swapgs () { asm ! ("swapgs") ; }
};
}
