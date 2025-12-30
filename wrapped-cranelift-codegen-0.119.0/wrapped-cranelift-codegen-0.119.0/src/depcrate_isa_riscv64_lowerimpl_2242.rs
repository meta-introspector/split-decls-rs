// Generated macro for impl_2242 (impl)
macro_rules! Depcrate_isa_riscv64_lowerimpl_2242 {
() => {
// Module: crate::isa::riscv64::lower
// Provides: {"impl_2242"}
// Dependencies: {}
impl LowerBackend for Riscv64Backend { type MInst = Inst ; fn lower (& self , ctx : & mut Lower < Inst > , ir_inst : IRInst) -> Option < InstOutput > { isle :: lower (ctx , self , ir_inst) } fn lower_branch (& self , ctx : & mut Lower < Inst > , ir_inst : IRInst , targets : & [MachLabel] ,) -> Option < () > { isle :: lower_branch (ctx , self , ir_inst , targets) } fn maybe_pinned_reg (& self) -> Option < Reg > { None } type FactFlowState = () ; }
};
}
