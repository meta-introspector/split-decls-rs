// Generated macro for impl_2104 (impl)
macro_rules! Depcrate_isa_riscv64_inst_emitimpl_2104 {
() => {
// Module: crate::isa::riscv64::inst::emit
// Provides: {"impl_2104"}
// Dependencies: {}
impl MachInstEmit for Inst { type State = EmitState ; type Info = EmitInfo ; fn emit (& self , sink : & mut MachBuffer < Inst > , emit_info : & Self :: Info , state : & mut EmitState) { if let Some (expected) = self . expected_vstate () { if state . vstate != EmitVState :: Known (* expected) { Inst :: VecSetState { rd : writable_zero_reg () , vstate : * expected , } . emit (sink , emit_info , state) ; } } let mut start_off = sink . cur_offset () ; let res = self . try_emit_compressed (sink , emit_info , state , & mut start_off) ; if res . is_none () { self . emit_uncompressed (sink , emit_info , state , & mut start_off) ; } if ! matches ! (self , Inst :: BrTable { .. } | Inst :: ReturnCall { .. } | Inst :: ReturnCallInd { .. }) { let end_off = sink . cur_offset () ; assert ! ((end_off - start_off) <= Inst :: worst_case_size () , "Inst:{:?} length:{} worst_case_size:{}" , self , end_off - start_off , Inst :: worst_case_size ()) ; } } fn pretty_print_inst (& self , state : & mut Self :: State) -> String { self . print_with_state (state) } }
};
}
