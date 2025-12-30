// Generated macro for read (function)
macro_rules! Depcrate_bits32_eflagsread {
() => {
// Module: crate::bits32::eflags
// Provides: {"read"}
// Dependencies: {}
# [cfg (target_arch = "x86")] # [inline (always)] pub unsafe fn read () -> EFlags { let r : u32 ; asm ! ("pushfl; popl {0}" , out (reg) r , options (att_syntax)) ; EFlags :: from_bits_truncate (r) }
};
}
