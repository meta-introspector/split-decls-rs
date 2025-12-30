// Generated macro for rbp (function)
macro_rules! Depcrate_bits64_registersrbp {
() => {
// Module: crate::bits64::registers
// Provides: {"rbp"}
// Dependencies: {}
# [doc = " Read the RBP register (base pointer register)."] # [inline (always)] pub fn rbp () -> u64 { let rbp : u64 ; unsafe { asm ! ("mov %rbp, {0}" , out (reg) rbp , options (att_syntax)) ; } rbp }
};
}
