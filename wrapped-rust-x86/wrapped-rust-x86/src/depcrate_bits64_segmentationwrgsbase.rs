// Generated macro for wrgsbase (function)
macro_rules! Depcrate_bits64_segmentationwrgsbase {
() => {
// Module: crate::bits64::segmentation
// Provides: {"wrgsbase"}
// Dependencies: {}
# [doc = " Write GS Segment Base"] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs FSGSBASE-Enable Bit (bit 16 of CR4) set."] # [cfg (target_arch = "x86_64")] pub unsafe fn wrgsbase (base : u64) { asm ! ("wrgsbase {0}" , in (reg) base , options (att_syntax)) ; }
};
}
