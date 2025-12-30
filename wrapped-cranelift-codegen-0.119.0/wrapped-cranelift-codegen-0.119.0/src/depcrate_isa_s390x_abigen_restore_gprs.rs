// Generated macro for gen_restore_gprs (function)
macro_rules! Depcrate_isa_s390x_abigen_restore_gprs {
() => {
// Module: crate::isa::s390x::abi
// Provides: {"gen_restore_gprs"}
// Dependencies: {}
fn gen_restore_gprs (call_conv : isa :: CallConv , frame_layout : & FrameLayout , callee_pop_size : u32 ,) -> SmallVec < [Inst ; 16] > { let mut insts = SmallVec :: new () ; let clobbered_gpr = get_clobbered_gprs (frame_layout) ; let stack_size = frame_layout . outgoing_args_size as i32 + frame_layout . clobber_size as i32 + frame_layout . fixed_frame_storage_size as i32 ; let implicit_sp_restore = callee_pop_size == 0 && (call_conv != isa :: CallConv :: Tail || frame_layout . incoming_args_size == 0) && clobbered_gpr . map_or (false , | (first , _) | { SImm20 :: maybe_from_i64 (8 * first as i64 + stack_size as i64) . is_some () }) ; if ! implicit_sp_restore { insts . extend (S390xMachineDeps :: gen_sp_reg_adjust (stack_size - callee_pop_size as i32 ,)) ; } if let Some ((first , mut last)) = clobbered_gpr { let mut reg = stack_reg () ; let mut offset = callee_pop_size as i64 + 8 * first as i64 ; if implicit_sp_restore { offset += stack_size as i64 - callee_pop_size as i64 ; last = 15 ; } if SImm20 :: maybe_from_i64 (offset) . is_none () { insts . extend (S390xMachineDeps :: gen_add_imm (call_conv , writable_gpr (first) , stack_reg () , offset as u32 ,)) ; reg = gpr (first) ; offset = 0 ; } insts . push (Inst :: LoadMultiple64 { rt : writable_gpr (first) , rt2 : writable_gpr (last) , mem : MemArg :: reg_plus_off (reg , offset , MemFlags :: trusted ()) , }) ; } insts }
};
}
