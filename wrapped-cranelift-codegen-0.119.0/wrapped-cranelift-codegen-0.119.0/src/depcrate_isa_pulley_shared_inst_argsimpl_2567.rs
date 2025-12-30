// Generated macro for impl_2567 (impl)
macro_rules! Depcrate_isa_pulley_shared_inst_argsimpl_2567 {
() => {
// Module: crate::isa::pulley_shared::inst::args
// Provides: {"impl_2567"}
// Dependencies: {}
impl AddrO32 { # [doc = " Implementation of regalloc for this addressing mode."] pub fn collect_operands (& mut self , collector : & mut impl OperandVisitor) { match self { AddrO32 :: Base { addr , offset : _ } => { collector . reg_use (addr) ; } } } }
};
}
