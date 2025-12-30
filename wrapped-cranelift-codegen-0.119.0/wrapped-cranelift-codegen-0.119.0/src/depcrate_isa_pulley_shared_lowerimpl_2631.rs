// Generated macro for impl_2631 (impl)
macro_rules! Depcrate_isa_pulley_shared_lowerimpl_2631 {
() => {
// Module: crate::isa::pulley_shared::lower
// Provides: {"impl_2631"}
// Dependencies: {}
impl < P > LowerBackend for PulleyBackend < P > where P : PulleyTargetKind , { type MInst = InstAndKind < P > ; fn lower (& self , ctx : & mut Lower < Self :: MInst > , ir_inst : ir :: Inst) -> Option < InstOutput > { isle :: lower (ctx , self , ir_inst) } fn lower_branch (& self , ctx : & mut Lower < Self :: MInst > , ir_inst : ir :: Inst , targets : & [MachLabel] ,) -> Option < () > { isle :: lower_branch (ctx , self , ir_inst , targets) } fn maybe_pinned_reg (& self) -> Option < Reg > { None } type FactFlowState = () ; }
};
}
