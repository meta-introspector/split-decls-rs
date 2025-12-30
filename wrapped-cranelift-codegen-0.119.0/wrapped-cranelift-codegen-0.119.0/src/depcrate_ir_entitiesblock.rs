// Generated macro for Block (struct)
macro_rules! Depcrate_ir_entitiesBlock {
() => {
// Module: crate::ir::entities
// Provides: {"Block"}
// Dependencies: {}
# [doc = " An opaque reference to a [basic block](https://en.wikipedia.org/wiki/Basic_block) in a"] # [doc = " [`Function`](super::function::Function)."] # [doc = ""] # [doc = " You can get a `Block` using"] # [doc = " [`FunctionBuilder::create_block`](https://docs.rs/cranelift-frontend/*/cranelift_frontend/struct.FunctionBuilder.html#method.create_block)"] # [doc = ""] # [doc = " While the order is stable, it is arbitrary and does not necessarily resemble the layout order."] # [derive (Copy , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct Block (u32) ;
};
}
