// Generated macro for Inst (struct)
macro_rules! Depcrate_ir_entitiesInst {
() => {
// Module: crate::ir::entities
// Provides: {"Inst"}
// Dependencies: {}
# [doc = " An opaque reference to an instruction in a [`Function`](super::Function)."] # [doc = ""] # [doc = " Most usage of `Inst` is internal. `Inst`ructions are returned by"] # [doc = " [`InstBuilder`](super::InstBuilder) instructions that do not return a"] # [doc = " [`Value`], such as control flow and trap instructions, as well as instructions that return a"] # [doc = " variable (potentially zero!) number of values, like call or call-indirect instructions. To get"] # [doc = " the `Value` of such instructions, use [`inst_results`](super::DataFlowGraph::inst_results) or"] # [doc = " its analogue in `cranelift_frontend::FuncBuilder`."] # [doc = ""] # [doc = " [inst_comment]: https://github.com/bjorn3/rustc_codegen_cranelift/blob/0f8814fd6da3d436a90549d4bb19b94034f2b19c/src/pretty_clif.rs"] # [doc = ""] # [doc = " While the order is stable, it is arbitrary and does not necessarily resemble the layout order."] # [derive (Copy , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct Inst (u32) ;
};
}
