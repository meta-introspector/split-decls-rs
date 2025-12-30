// Generated macro for impl_2601 (impl)
macro_rules! Depcrate_isa_pulley_shared_inst_emitimpl_2601 {
() => {
// Module: crate::isa::pulley_shared::inst::emit
// Provides: {"impl_2601"}
// Dependencies: {}
impl < P > MachInstEmit for InstAndKind < P > where P : PulleyTargetKind , { type State = EmitState < P > ; type Info = EmitInfo ; fn emit (& self , sink : & mut MachBuffer < Self > , emit_info : & Self :: Info , state : & mut Self :: State) { let mut start = sink . cur_offset () ; pulley_emit (self , sink , emit_info , state , & mut start) ; let end = sink . cur_offset () ; assert ! ((end - start) <= InstAndKind ::< P >:: worst_case_size () , "encoded inst {self:?} longer than worst-case size: length: {}, Inst::worst_case_size() = {}" , end - start , InstAndKind ::< P >:: worst_case_size ()) ; } fn pretty_print_inst (& self , state : & mut Self :: State) -> String { self . print_with_state (state) } }
};
}
