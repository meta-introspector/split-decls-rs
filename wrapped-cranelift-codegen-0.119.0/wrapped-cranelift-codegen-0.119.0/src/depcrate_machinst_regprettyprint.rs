// Generated macro for PrettyPrint (trait)
macro_rules! Depcrate_machinst_regPrettyPrint {
() => {
// Module: crate::machinst::reg
// Provides: {"PrettyPrint"}
// Dependencies: {}
# [doc = " Pretty-print part of a disassembly, with knowledge of"] # [doc = " operand/instruction size, and optionally with regalloc"] # [doc = " results. This can be used, for example, to print either `rax` or"] # [doc = " `eax` for the register by those names on x86-64, depending on a"] # [doc = " 64- or 32-bit context."] pub trait PrettyPrint { fn pretty_print (& self , size_bytes : u8) -> String ; fn pretty_print_default (& self) -> String { self . pretty_print (0) } }
};
}
