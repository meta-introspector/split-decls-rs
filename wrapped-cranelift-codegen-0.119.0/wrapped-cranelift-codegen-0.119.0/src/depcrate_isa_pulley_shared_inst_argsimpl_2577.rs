// Generated macro for impl_2577 (impl)
macro_rules! Depcrate_isa_pulley_shared_inst_argsimpl_2577 {
() => {
// Module: crate::isa::pulley_shared::inst::args
// Provides: {"impl_2577"}
// Dependencies: {}
impl AddrG32 { # [doc = " Implementation of regalloc for this addressing mode."] pub fn collect_operands (& mut self , collector : & mut impl OperandVisitor) { match self { AddrG32 :: RegisterBound { host_heap_base , host_heap_bound , wasm_addr , offset : _ , } => { collector . reg_use (host_heap_base) ; collector . reg_use (host_heap_bound) ; collector . reg_use (wasm_addr) ; } } } }
};
}
