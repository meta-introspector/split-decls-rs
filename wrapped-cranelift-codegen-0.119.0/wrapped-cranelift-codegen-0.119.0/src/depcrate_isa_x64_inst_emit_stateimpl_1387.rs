// Generated macro for impl_1387 (impl)
macro_rules! Depcrate_isa_x64_inst_emit_stateimpl_1387 {
() => {
// Module: crate::isa::x64::inst::emit_state
// Provides: {"impl_1387"}
// Dependencies: {}
impl EmitState { pub (crate) fn take_stack_map (& mut self) -> Option < ir :: UserStackMap > { self . user_stack_map . take () } pub (crate) fn clear_post_insn (& mut self) { self . user_stack_map = None ; } }
};
}
