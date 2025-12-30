// Generated macro for wrfsbase (function)
macro_rules! Depcrate_bits64_segmentationwrfsbase {
() => {
// Module: crate::bits64::segmentation
// Provides: {"wrfsbase"}
// Dependencies: {}
# [doc = " Write FS Segment Base"] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs FSGSBASE-Enable Bit (bit 16 of CR4) set."] # [cfg (target_arch = "x86_64")] pub unsafe fn wrfsbase (base : u64) { asm ! ("wrfsbase {0}" , in (reg) base , options (att_syntax)) ; }
};
}
