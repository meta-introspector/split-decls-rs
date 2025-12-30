// Generated macro for emit_return_call_common_sequence (function)
macro_rules! Depcrate_isa_aarch64_inst_emitemit_return_call_common_sequence {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"emit_return_call_common_sequence"}
// Dependencies: {}
fn emit_return_call_common_sequence < T > (sink : & mut MachBuffer < Inst > , emit_info : & EmitInfo , state : & mut EmitState , info : & ReturnCallInfo < T > ,) { for inst in AArch64MachineDeps :: gen_clobber_restore (CallConv :: Tail , & emit_info . 0 , state . frame_layout ()) { inst . emit (sink , emit_info , state) ; } let setup_area_size = state . frame_layout () . setup_area_size ; if setup_area_size > 0 { Inst :: LoadP64 { rt : writable_fp_reg () , rt2 : writable_link_reg () , mem : PairAMode :: SPPostIndexed { simm7 : SImm7Scaled :: maybe_from_i64 (i64 :: from (setup_area_size) , types :: I64) . unwrap () , } , flags : MemFlags :: trusted () , } . emit (sink , emit_info , state) ; } let incoming_args_diff = state . frame_layout () . tail_args_size - info . new_stack_arg_size ; if incoming_args_diff > 0 { for inst in AArch64MachineDeps :: gen_sp_reg_adjust (i32 :: try_from (incoming_args_diff) . unwrap ()) { inst . emit (sink , emit_info , state) ; } } if let Some (key) = info . key { sink . put4 (key . enc_auti_hint ()) ; } }
};
}
