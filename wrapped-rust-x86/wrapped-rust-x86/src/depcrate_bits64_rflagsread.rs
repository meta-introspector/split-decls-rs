// Generated macro for read (function)
macro_rules! Depcrate_bits64_rflagsread {
() => {
// Module: crate::bits64::rflags
// Provides: {"read"}
// Dependencies: {}
# [cfg (target_arch = "x86_64")] # [inline (always)] pub fn read () -> RFlags { let r : u64 ; unsafe { asm ! ("pushfq; popq {0}" , out (reg) r , options (att_syntax)) } ; RFlags :: from_bits_truncate (r) }
};
}
