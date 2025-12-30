// Generated macro for impl_2440 (impl)
macro_rules! Depcrate_isa_s390x_inst_emitimpl_2440 {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"impl_2440"}
// Dependencies: {}
impl MachInstEmit for Inst { type State = EmitState ; type Info = EmitInfo ; fn emit (& self , sink : & mut MachBuffer < Inst > , emit_info : & Self :: Info , state : & mut EmitState) { self . emit_with_alloc_consumer (sink , emit_info , state) } fn pretty_print_inst (& self , state : & mut EmitState) -> String { self . print_with_state (state) } }
};
}
