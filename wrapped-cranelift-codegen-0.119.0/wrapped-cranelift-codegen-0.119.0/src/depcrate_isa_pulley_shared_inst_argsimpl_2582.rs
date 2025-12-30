// Generated macro for impl_2582 (impl)
macro_rules! Depcrate_isa_pulley_shared_inst_argsimpl_2582 {
() => {
// Module: crate::isa::pulley_shared::inst::args
// Provides: {"impl_2582"}
// Dependencies: {}
impl AddrG32Bne { # [doc = " Implementation of regalloc for this addressing mode."] pub fn collect_operands (& mut self , collector : & mut impl OperandVisitor) { match self { AddrG32Bne :: BoundNe { host_heap_base , host_heap_bound_addr , host_heap_bound_offset : _ , wasm_addr , offset : _ , } => { collector . reg_use (host_heap_base) ; collector . reg_use (host_heap_bound_addr) ; collector . reg_use (wasm_addr) ; } } } }
};
}
