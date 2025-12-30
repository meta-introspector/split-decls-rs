// Generated macro for impl_2474 (impl)
macro_rules! Depcrate_isa_s390x_lowerimpl_2474 {
() => {
// Module: crate::isa::s390x::lower
// Provides: {"impl_2474"}
// Dependencies: {}
impl LowerBackend for S390xBackend { type MInst = Inst ; fn lower (& self , ctx : & mut Lower < Inst > , ir_inst : IRInst) -> Option < InstOutput > { isle :: lower (ctx , self , ir_inst) } fn lower_branch (& self , ctx : & mut Lower < Inst > , ir_inst : IRInst , targets : & [MachLabel] ,) -> Option < () > { isle :: lower_branch (ctx , self , ir_inst , targets) } type FactFlowState = () ; }
};
}
