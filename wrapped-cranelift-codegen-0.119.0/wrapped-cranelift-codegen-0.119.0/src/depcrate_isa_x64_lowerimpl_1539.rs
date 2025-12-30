// Generated macro for impl_1539 (impl)
macro_rules! Depcrate_isa_x64_lowerimpl_1539 {
() => {
// Module: crate::isa::x64::lower
// Provides: {"impl_1539"}
// Dependencies: {}
impl LowerBackend for X64Backend { type MInst = Inst ; fn lower (& self , ctx : & mut Lower < Inst > , ir_inst : IRInst) -> Option < InstOutput > { isle :: lower (ctx , self , ir_inst) } fn lower_branch (& self , ctx : & mut Lower < Inst > , ir_inst : IRInst , targets : & [MachLabel] ,) -> Option < () > { isle :: lower_branch (ctx , self , ir_inst , targets) } fn maybe_pinned_reg (& self) -> Option < Reg > { Some (regs :: pinned_reg ()) } fn check_fact (& self , ctx : & FactContext < '_ > , vcode : & mut VCode < Self :: MInst > , inst : InsnIndex , state : & mut pcc :: FactFlowState ,) -> PccResult < () > { pcc :: check (ctx , vcode , inst , state) } type FactFlowState = pcc :: FactFlowState ; }
};
}
