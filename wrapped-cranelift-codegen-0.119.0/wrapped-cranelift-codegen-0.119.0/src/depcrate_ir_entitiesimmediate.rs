// Generated macro for Immediate (struct)
macro_rules! Depcrate_ir_entitiesImmediate {
() => {
// Module: crate::ir::entities
// Provides: {"Immediate"}
// Dependencies: {}
# [doc = " An opaque reference to an immediate."] # [doc = ""] # [doc = " Some immediates (e.g. SIMD shuffle masks) are too large to store in the"] # [doc = " [`InstructionData`](super::instructions::InstructionData) struct and therefore must be"] # [doc = " tracked separately in [`DataFlowGraph::immediates`](super::dfg::DataFlowGraph). `Immediate`"] # [doc = " provides a way to reference values stored there."] # [doc = ""] # [doc = " While the order is stable, it is arbitrary."] # [derive (Copy , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct Immediate (u32) ;
};
}
