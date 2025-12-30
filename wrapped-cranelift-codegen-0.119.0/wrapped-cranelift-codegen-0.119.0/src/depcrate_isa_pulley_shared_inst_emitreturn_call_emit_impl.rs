// Generated macro for return_call_emit_impl (function)
macro_rules! Depcrate_isa_pulley_shared_inst_emitreturn_call_emit_impl {
() => {
// Module: crate::isa::pulley_shared::inst::emit
// Provides: {"return_call_emit_impl"}
// Dependencies: {}
# [doc = " This should not be called directly, Instead prefer to call [emit_return_call_common_sequence]."] fn return_call_emit_impl < T , P > (sink : & mut MachBuffer < InstAndKind < P > > , emit_info : & EmitInfo , state : & mut EmitState < P > , info : & ReturnCallInfo < T > ,) where P : PulleyTargetKind , { let epilogue = < PulleyMachineDeps < P > > :: gen_epilogue_frame_restore (emit_info . call_conv , & emit_info . shared_flags , & emit_info . isa_flags , & state . frame_layout ,) ; for inst in epilogue { inst . emit (sink , emit_info , state) ; } let incoming_args_diff = i64 :: from (state . frame_layout () . tail_args_size - info . new_stack_arg_size) ; if incoming_args_diff != 0 { let amt = i32 :: try_from (incoming_args_diff) . unwrap () ; for inst in PulleyMachineDeps :: < P > :: gen_sp_reg_adjust (amt) { < InstAndKind < P > > :: from (inst) . emit (sink , emit_info , state) ; } } }
};
}
