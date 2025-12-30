// Generated macro for impl_2437 (impl)
macro_rules! Depcrate_isa_s390x_inst_emitimpl_2437 {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"impl_2437"}
// Dependencies: {}
impl EmitState { fn take_stack_map (& mut self) -> Option < ir :: UserStackMap > { self . user_stack_map . take () } fn clear_post_insn (& mut self) { self . user_stack_map = None ; } }
};
}
