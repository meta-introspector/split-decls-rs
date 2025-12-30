// Generated macro for rsp (function)
macro_rules! Depcrate_bits64_registersrsp {
() => {
// Module: crate::bits64::registers
// Provides: {"rsp"}
// Dependencies: {}
# [doc = " Read the RSP register (stack pointer register)."] # [inline (always)] pub fn rsp () -> u64 { let rsp : u64 ; unsafe { asm ! ("mov %rsp, {0}" , out (reg) rsp , options (att_syntax)) ; } rsp }
};
}
