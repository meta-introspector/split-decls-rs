// Generated macro for SigRef (struct)
macro_rules! Depcrate_ir_entitiesSigRef {
() => {
// Module: crate::ir::entities
// Provides: {"SigRef"}
// Dependencies: {}
# [doc = " An opaque reference to a function [`Signature`](super::Signature)."] # [doc = ""] # [doc = " `SigRef`s are used to declare a function with"] # [doc = " [`FunctionBuilder::import_function`](https://docs.rs/cranelift-frontend/*/cranelift_frontend/struct.FunctionBuilder.html#method.import_function)"] # [doc = " as well as to make an [indirect function call](super::InstBuilder::call_indirect)."] # [doc = ""] # [doc = " `SigRef`s can be created with"] # [doc = " [`FunctionBuilder::import_signature`](https://docs.rs/cranelift-frontend/*/cranelift_frontend/struct.FunctionBuilder.html#method.import_signature)."] # [doc = ""] # [doc = " You can retrieve the [`Signature`](super::Signature) that was used to create a `SigRef` with"] # [doc = " [`FunctionBuilder::signature`](https://docs.rs/cranelift-frontend/*/cranelift_frontend/struct.FunctionBuilder.html#method.signature) or"] # [doc = " [`func.dfg.signatures`](super::dfg::DataFlowGraph::signatures)."] # [doc = ""] # [doc = " While the order is stable, it is arbitrary."] # [derive (Copy , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct SigRef (u32) ;
};
}
