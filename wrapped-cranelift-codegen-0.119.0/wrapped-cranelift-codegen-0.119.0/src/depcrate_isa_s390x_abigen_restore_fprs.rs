// Generated macro for gen_restore_fprs (function)
macro_rules! Depcrate_isa_s390x_abigen_restore_fprs {
() => {
// Module: crate::isa::s390x::abi
// Provides: {"gen_restore_fprs"}
// Dependencies: {}
fn gen_restore_fprs (frame_layout : & FrameLayout) -> SmallVec < [Inst ; 16] > { let mut insts = SmallVec :: new () ; let clobbered_fpr = get_clobbered_fprs (frame_layout) ; for (i , reg) in clobbered_fpr . iter () . enumerate () { insts . push (Inst :: VecLoadLaneUndef { size : 64 , rd : Writable :: from_reg (reg . to_reg () . into ()) , mem : MemArg :: reg_plus_off (stack_reg () , (i * 8) as i64 + frame_layout . outgoing_args_size as i64 + frame_layout . fixed_frame_storage_size as i64 , MemFlags :: trusted () ,) , lane_imm : 0 , }) ; } insts }
};
}
