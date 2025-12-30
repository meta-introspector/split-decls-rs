// Generated macro for impl_2963 (impl)
macro_rules! Depcrate_egraphimpl_2963 {
() => {
// Module: crate::egraph
// Provides: {"impl_2963"}
// Dependencies: {}
impl NewOrExistingInst { fn get_inst_key < 'a > (& 'a self , dfg : & 'a DataFlowGraph) -> (Type , InstructionData) { match self { NewOrExistingInst :: New (data , ty) => (* ty , * data) , NewOrExistingInst :: Existing (inst) => { let ty = dfg . ctrl_typevar (* inst) ; (ty , dfg . insts [* inst]) } } } }
};
}
