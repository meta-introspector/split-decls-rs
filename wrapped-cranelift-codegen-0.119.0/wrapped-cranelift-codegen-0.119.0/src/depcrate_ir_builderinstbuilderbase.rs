// Generated macro for InstBuilderBase (trait)
macro_rules! Depcrate_ir_builderInstBuilderBase {
() => {
// Module: crate::ir::builder
// Provides: {"InstBuilderBase"}
// Dependencies: {}
# [doc = " Base trait for instruction builders."] # [doc = ""] # [doc = " The `InstBuilderBase` trait provides the basic functionality required by the methods of the"] # [doc = " generated `InstBuilder` trait. These methods should not normally be used directly. Use the"] # [doc = " methods in the `InstBuilder` trait instead."] # [doc = ""] # [doc = " Any data type that implements `InstBuilderBase` also gets all the methods of the `InstBuilder`"] # [doc = " trait."] pub trait InstBuilderBase < 'f > : Sized { # [doc = " Get an immutable reference to the data flow graph that will hold the constructed"] # [doc = " instructions."] fn data_flow_graph (& self) -> & DataFlowGraph ; # [doc = " Get a mutable reference to the data flow graph that will hold the constructed"] # [doc = " instructions."] fn data_flow_graph_mut (& mut self) -> & mut DataFlowGraph ; # [doc = " Insert an instruction and return a reference to it, consuming the builder."] # [doc = ""] # [doc = " The result types may depend on a controlling type variable. For non-polymorphic"] # [doc = " instructions with multiple results, pass `INVALID` for the `ctrl_typevar` argument."] fn build (self , data : InstructionData , ctrl_typevar : Type) -> (Inst , & 'f mut DataFlowGraph) ; }
};
}
