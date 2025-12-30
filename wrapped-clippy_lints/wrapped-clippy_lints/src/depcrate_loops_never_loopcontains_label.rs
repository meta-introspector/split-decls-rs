// Generated macro for contains_label (function)
macro_rules! Depcrate_loops_never_loopcontains_label {
() => {
// Module: crate::loops::never_loop
// Provides: {"contains_label"}
// Dependencies: {}
fn contains_label (asm : & InlineAsm < '_ >) -> bool { asm . operands . iter () . any (| (op , _span) | matches ! (op , InlineAsmOperand :: Label { .. })) }
};
}
