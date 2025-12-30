// Generated macro for impl_526 (impl)
macro_rules! Depcrate_ir_builderimpl_526 {
() => {
// Module: crate::ir::builder
// Provides: {"impl_526"}
// Dependencies: {}
impl < 'f , IIB : InstInserterBase < 'f > > InstBuilderBase < 'f > for InsertBuilder < 'f , IIB > { fn data_flow_graph (& self) -> & DataFlowGraph { self . inserter . data_flow_graph () } fn data_flow_graph_mut (& mut self) -> & mut DataFlowGraph { self . inserter . data_flow_graph_mut () } fn build (mut self , data : InstructionData , ctrl_typevar : Type) -> (Inst , & 'f mut DataFlowGraph) { let inst ; { let dfg = self . inserter . data_flow_graph_mut () ; inst = dfg . make_inst (data) ; dfg . make_inst_results (inst , ctrl_typevar) ; } (inst , self . inserter . insert_built_inst (inst)) } }
};
}
