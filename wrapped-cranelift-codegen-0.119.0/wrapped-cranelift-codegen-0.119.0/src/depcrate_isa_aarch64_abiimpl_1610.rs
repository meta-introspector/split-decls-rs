// Generated macro for impl_1610 (impl)
macro_rules! Depcrate_isa_aarch64_abiimpl_1610 {
() => {
// Module: crate::isa::aarch64::abi
// Provides: {"impl_1610"}
// Dependencies: {}
impl AArch64MachineDeps { fn gen_probestack_unroll (insts : & mut SmallInstVec < Inst > , guard_size : u32 , probe_count : u32) { for _ in 0 .. probe_count { insts . extend (Self :: gen_sp_reg_adjust (- (guard_size as i32))) ; insts . push (Inst :: gen_store (AMode :: SPOffset { off : 0 } , zero_reg () , I32 , MemFlags :: trusted () ,)) ; } insts . extend (Self :: gen_sp_reg_adjust ((guard_size * probe_count) as i32)) ; } fn gen_probestack_loop (insts : & mut SmallInstVec < Inst > , frame_size : u32 , guard_size : u32) { let start = writable_spilltmp_reg () ; let end = writable_tmp2_reg () ; insts . extend (Inst :: load_constant (start , 0 , & mut | _ | start)) ; insts . extend (Inst :: load_constant (end , frame_size . into () , & mut | _ | end)) ; insts . push (Inst :: StackProbeLoop { start , end : end . to_reg () , step : Imm12 :: maybe_from_u64 (guard_size . into ()) . unwrap () , }) ; } }
};
}
