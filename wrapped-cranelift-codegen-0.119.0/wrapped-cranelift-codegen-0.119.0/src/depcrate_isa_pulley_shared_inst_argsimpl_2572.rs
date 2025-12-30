// Generated macro for impl_2572 (impl)
macro_rules! Depcrate_isa_pulley_shared_inst_argsimpl_2572 {
() => {
// Module: crate::isa::pulley_shared::inst::args
// Provides: {"impl_2572"}
// Dependencies: {}
impl AddrZ { # [doc = " Implementation of regalloc for this addressing mode."] pub fn collect_operands (& mut self , collector : & mut impl OperandVisitor) { match self { AddrZ :: Base { addr , offset : _ } => { collector . reg_use (addr) ; } } } }
};
}
