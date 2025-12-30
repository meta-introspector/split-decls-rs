// Generated macro for impl_1870 (impl)
macro_rules! Depcrate_isa_aarch64_lowerimpl_1870 {
() => {
// Module: crate::isa::aarch64::lower
// Provides: {"impl_1870"}
// Dependencies: {}
impl LowerBackend for AArch64Backend { type MInst = Inst ; fn lower (& self , ctx : & mut Lower < Inst > , ir_inst : IRInst) -> Option < InstOutput > { isle :: lower (ctx , self , ir_inst) } fn lower_branch (& self , ctx : & mut Lower < Inst > , ir_inst : IRInst , targets : & [MachLabel] ,) -> Option < () > { isle :: lower_branch (ctx , self , ir_inst , targets) } fn maybe_pinned_reg (& self) -> Option < Reg > { Some (regs :: pinned_reg ()) } fn check_fact (& self , ctx : & FactContext < '_ > , vcode : & mut VCode < Self :: MInst > , inst : InsnIndex , state : & mut pcc :: FactFlowState ,) -> PccResult < () > { pcc :: check (ctx , vcode , inst , state) } type FactFlowState = pcc :: FactFlowState ; }
};
}
