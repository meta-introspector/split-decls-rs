// Generated macro for impl_1822 (impl)
macro_rules! Depcrate_isa_aarch64_inst_emitimpl_1822 {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"impl_1822"}
// Dependencies: {}
impl EmitState { fn take_stack_map (& mut self) -> Option < ir :: UserStackMap > { self . user_stack_map . take () } fn clear_post_insn (& mut self) { self . user_stack_map = None ; } }
};
}
