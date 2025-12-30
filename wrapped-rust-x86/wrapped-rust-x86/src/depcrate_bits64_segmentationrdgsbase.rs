// Generated macro for rdgsbase (function)
macro_rules! Depcrate_bits64_segmentationrdgsbase {
() => {
// Module: crate::bits64::segmentation
// Provides: {"rdgsbase"}
// Dependencies: {}
# [doc = " Read GS Segment Base"] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs FSGSBASE-Enable Bit (bit 16 of CR4) set."] # [cfg (target_arch = "x86_64")] pub unsafe fn rdgsbase () -> u64 { let gs_base : u64 ; asm ! ("rdgsbase {0}" , out (reg) gs_base , options (att_syntax)) ; gs_base }
};
}
