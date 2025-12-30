// Generated macro for impl_2302 (impl)
macro_rules! Depcrate_isa_s390x_abiimpl_2302 {
() => {
// Module: crate::isa::s390x::abi
// Provides: {"impl_2302"}
// Dependencies: {}
impl S390xMachineDeps { pub fn gen_tail_epilogue (frame_layout : & FrameLayout , callee_pop_size : u32 , target_reg : Option < & mut Reg > ,) -> SmallVec < [Inst ; 16] > { let mut insts = SmallVec :: new () ; let call_conv = isa :: CallConv :: Tail ; insts . extend (gen_restore_fprs (frame_layout)) ; if let Some (reg) = target_reg { if is_reg_saved_in_prologue (call_conv , reg . to_real_reg () . unwrap ()) { insts . push (Inst :: Mov64 { rd : writable_gpr (1) , rm : * reg , }) ; * reg = gpr (1) ; } } insts . extend (gen_restore_gprs (call_conv , frame_layout , callee_pop_size)) ; insts } }
};
}
