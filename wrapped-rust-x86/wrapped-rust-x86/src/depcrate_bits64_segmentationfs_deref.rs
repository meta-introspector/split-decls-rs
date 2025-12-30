// Generated macro for fs_deref (macro)
macro_rules! Depcrate_bits64_segmentationfs_deref {
() => {
// Module: crate::bits64::segmentation
// Provides: {"fs_deref"}
// Dependencies: {}
# [doc = " \"Dereferences\" the fs register at `offset`."] # [doc = ""] # [doc = " # Safety"] # [doc = " - Offset needs to be within valid memory relative to what the fs register"] # [doc = "   points to."] # [cfg (target_arch = "x86_64")] # [macro_export] macro_rules ! fs_deref { ($ offset : expr) => { { let fs : u64 ; core :: arch :: asm ! ("movq %fs:{offset}, {result}" , offset = const ($ offset) , result = out (reg) fs , options (att_syntax)) ; fs } } ; }
};
}
