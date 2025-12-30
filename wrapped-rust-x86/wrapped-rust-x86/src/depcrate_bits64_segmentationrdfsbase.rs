// Generated macro for rdfsbase (function)
macro_rules! Depcrate_bits64_segmentationrdfsbase {
() => {
// Module: crate::bits64::segmentation
// Provides: {"rdfsbase"}
// Dependencies: {}
# [doc = " Read FS Segment Base"] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs FSGSBASE-Enable Bit (bit 16 of CR4) set."] # [cfg (target_arch = "x86_64")] pub unsafe fn rdfsbase () -> u64 { let fs_base : u64 ; asm ! ("rdfsbase {0}" , out (reg) fs_base , options (att_syntax)) ; fs_base }
};
}
