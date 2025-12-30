// Generated macro for InstInserterBase (trait)
macro_rules! Depcrate_ir_builderInstInserterBase {
() => {
// Module: crate::ir::builder
// Provides: {"InstInserterBase"}
// Dependencies: {}
# [doc = " Base trait for instruction inserters."] # [doc = ""] # [doc = " This is an alternative base trait for an instruction builder to implement."] # [doc = ""] # [doc = " An instruction inserter can be adapted into an instruction builder by wrapping it in an"] # [doc = " `InsertBuilder`. This provides some common functionality for instruction builders that insert"] # [doc = " new instructions, as opposed to the `ReplaceBuilder` which overwrites existing instructions."] pub trait InstInserterBase < 'f > : Sized { # [doc = " Get an immutable reference to the data flow graph."] fn data_flow_graph (& self) -> & DataFlowGraph ; # [doc = " Get a mutable reference to the data flow graph."] fn data_flow_graph_mut (& mut self) -> & mut DataFlowGraph ; # [doc = " Insert a new instruction which belongs to the DFG."] fn insert_built_inst (self , inst : Inst) -> & 'f mut DataFlowGraph ; }
};
}
