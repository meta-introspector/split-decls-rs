// Generated macro for stack_jmp (function)
macro_rules! Depcrate_bits32stack_jmp {
() => {
// Module: crate::bits32
// Provides: {"stack_jmp"}
// Dependencies: {}
# [cfg (target_arch = "x86")] # [inline (always)] pub unsafe fn stack_jmp (stack : * mut () , ip : * const ()) -> ! { asm ! ("movl {0}, %esp; jmp {1}" , in (reg) stack , in (reg) ip , options (att_syntax)) ; loop { } }
};
}
