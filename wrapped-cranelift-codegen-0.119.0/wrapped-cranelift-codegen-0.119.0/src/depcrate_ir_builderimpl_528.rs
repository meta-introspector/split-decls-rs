// Generated macro for impl_528 (impl)
macro_rules! Depcrate_ir_builderimpl_528 {
() => {
// Module: crate::ir::builder
// Provides: {"impl_528"}
// Dependencies: {}
impl < 'f , IIB , Array > InstBuilderBase < 'f > for InsertReuseBuilder < 'f , IIB , Array > where IIB : InstInserterBase < 'f > , Array : AsRef < [Option < Value >] > , { fn data_flow_graph (& self) -> & DataFlowGraph { self . inserter . data_flow_graph () } fn data_flow_graph_mut (& mut self) -> & mut DataFlowGraph { self . inserter . data_flow_graph_mut () } fn build (mut self , data : InstructionData , ctrl_typevar : Type) -> (Inst , & 'f mut DataFlowGraph) { let inst ; { let dfg = self . inserter . data_flow_graph_mut () ; inst = dfg . make_inst (data) ; let ru = self . reuse . as_ref () . iter () . cloned () ; dfg . make_inst_results_reusing (inst , ctrl_typevar , ru) ; } (inst , self . inserter . insert_built_inst (inst)) } }
};
}
