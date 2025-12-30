// Generated macro for gs_deref (macro)
macro_rules! Depcrate_bits64_segmentationgs_deref {
() => {
// Module: crate::bits64::segmentation
// Provides: {"gs_deref"}
// Dependencies: {}
# [doc = " \"Dereferences\" the gs register at `offset`."] # [doc = ""] # [doc = " # Safety"] # [doc = " - Offset needs to be within valid memory relative to what the gs register"] # [doc = "   points to."] # [cfg (target_arch = "x86_64")] # [macro_export] macro_rules ! gs_deref { ($ offset : expr) => { { let gs : u64 ; core :: arch :: asm ! ("movq %gs:{offset}, {result}" , offset = const ($ offset) , result = out (reg) gs , options (att_syntax)) ; gs } } ; }
};
}
