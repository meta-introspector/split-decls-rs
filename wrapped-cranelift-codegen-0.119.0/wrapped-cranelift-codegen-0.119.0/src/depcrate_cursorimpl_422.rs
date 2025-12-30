// Generated macro for impl_422 (impl)
macro_rules! Depcrate_cursorimpl_422 {
() => {
// Module: crate::cursor
// Provides: {"impl_422"}
// Dependencies: {}
impl < 'c , 'f > ir :: InstInserterBase < 'c > for & 'c mut FuncCursor < 'f > { fn data_flow_graph (& self) -> & ir :: DataFlowGraph { & self . func . dfg } fn data_flow_graph_mut (& mut self) -> & mut ir :: DataFlowGraph { & mut self . func . dfg } fn insert_built_inst (self , inst : ir :: Inst) -> & 'c mut ir :: DataFlowGraph { self . insert_inst (inst) ; if ! self . srcloc . is_default () { self . func . set_srcloc (inst , self . srcloc) ; } & mut self . func . dfg } }
};
}
