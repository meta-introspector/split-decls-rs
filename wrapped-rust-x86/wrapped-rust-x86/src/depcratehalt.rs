// Generated macro for halt (function)
macro_rules! Depcratehalt {
() => {
// Module: crate
// Provides: {"halt"}
// Dependencies: {}
# [doc = " Stops instruction execution and places the processor in a HALT state."] # [doc = ""] # [doc = " An enabled interrupt (including NMI and SMI), a debug exception, the BINIT#"] # [doc = " signal, the INIT# signal, or the RESET# signal will resume execution. If an"] # [doc = " interrupt (including NMI) is used to resume execution after a HLT instruction,"] # [doc = " the saved instruction pointer (CS:EIP) points to the instruction following"] # [doc = " the HLT instruction."] # [doc = ""] # [doc = " # Safety"] # [doc = " Will cause a general protection fault if used outside of ring 0."] # [inline (always)] pub unsafe fn halt () { asm ! ("hlt" , options (att_syntax , nomem , nostack)) ; }
};
}
