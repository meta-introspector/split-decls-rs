// Generated macro for impl_1948 (impl)
macro_rules! Depcrate_isa_riscv64_abiimpl_1948 {
() => {
// Module: crate::isa::riscv64::abi
// Provides: {"impl_1948"}
// Dependencies: {}
impl Riscv64MachineDeps { fn gen_probestack_unroll (insts : & mut SmallInstVec < Inst > , tmp : Writable < Reg > , guard_size : u32 , probe_count : u32 ,) { insts . extend (Inst :: load_constant_u64 (tmp , (- (guard_size as i64)) as u64)) ; for _ in 0 .. probe_count { insts . push (Inst :: AluRRR { alu_op : AluOPRRR :: Add , rd : writable_stack_reg () , rs1 : stack_reg () , rs2 : tmp . to_reg () , }) ; insts . push (Inst :: gen_store (AMode :: SPOffset (0) , zero_reg () , I32 , MemFlags :: trusted () ,)) ; } insts . extend (Self :: gen_sp_reg_adjust ((guard_size * probe_count) as i32)) ; } }
};
}
