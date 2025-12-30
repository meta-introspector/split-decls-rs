// Generated macro for rip (function)
macro_rules! Depcrate_bits64_registersrip {
() => {
// Module: crate::bits64::registers
// Provides: {"rip"}
// Dependencies: {}
# [doc = " Read the RIP register (instruction pointer)."] # [inline (always)] pub fn rip () -> u64 { let rip : u64 ; unsafe { asm ! ("leaq 0(%rip), {0}" , out (reg) rip , options (att_syntax)) ; } rip }
};
}
