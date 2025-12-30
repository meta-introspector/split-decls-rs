// Generated macro for impl_531 (impl)
macro_rules! Depcrate_ir_builderimpl_531 {
() => {
// Module: crate::ir::builder
// Provides: {"impl_531"}
// Dependencies: {}
impl < 'f > InstBuilderBase < 'f > for ReplaceBuilder < 'f > { fn data_flow_graph (& self) -> & DataFlowGraph { self . dfg } fn data_flow_graph_mut (& mut self) -> & mut DataFlowGraph { self . dfg } fn build (self , data : InstructionData , ctrl_typevar : Type) -> (Inst , & 'f mut DataFlowGraph) { self . dfg . insts [self . inst] = data ; if ! self . dfg . has_results (self . inst) { self . dfg . make_inst_results (self . inst , ctrl_typevar) ; } (self . inst , self . dfg) } }
};
}
