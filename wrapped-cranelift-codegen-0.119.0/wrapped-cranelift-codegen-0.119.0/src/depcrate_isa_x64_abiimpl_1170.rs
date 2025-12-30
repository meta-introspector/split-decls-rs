// Generated macro for impl_1170 (impl)
macro_rules! Depcrate_isa_x64_abiimpl_1170 {
() => {
// Module: crate::isa::x64::abi
// Provides: {"impl_1170"}
// Dependencies: {}
impl X64ABIMachineSpec { fn gen_probestack_unroll (insts : & mut SmallInstVec < Inst > , guard_size : u32 , probe_count : u32) { insts . reserve (probe_count as usize) ; for _ in 0 .. probe_count { insts . extend (Self :: gen_sp_reg_adjust (- (guard_size as i32))) ; insts . push (Inst :: store (I32 , regs :: rsp () , Amode :: imm_reg (0 , regs :: rsp ()) ,)) ; } insts . extend (Self :: gen_sp_reg_adjust ((guard_size * probe_count) as i32)) ; } fn gen_probestack_loop (insts : & mut SmallInstVec < Inst > , _call_conv : isa :: CallConv , frame_size : u32 , guard_size : u32 ,) { let tmp = regs :: r11 () ; debug_assert ! ({ let real_reg = tmp . to_real_reg () . unwrap () ; ! is_callee_save_systemv (real_reg , false) && ! is_callee_save_fastcall (real_reg , false) }) ; insts . push (Inst :: StackProbeLoop { tmp : Writable :: from_reg (tmp) , frame_size , guard_size , }) ; } }
};
}
