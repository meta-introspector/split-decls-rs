// Generated macro for StackSlot (struct)
macro_rules! Depcrate_ir_entitiesStackSlot {
() => {
// Module: crate::ir::entities
// Provides: {"StackSlot"}
// Dependencies: {}
# [doc = " An opaque reference to a stack slot."] # [doc = ""] # [doc = " Stack slots represent an address on the"] # [doc = " [call stack](https://en.wikipedia.org/wiki/Call_stack)."] # [doc = ""] # [doc = " `StackSlot`s can be created with"] # [doc = " [`FunctionBuilder::create_sized_stack_slot`](https://docs.rs/cranelift-frontend/*/cranelift_frontend/struct.FunctionBuilder.html#method.create_sized_stack_slot)"] # [doc = " or"] # [doc = " [`FunctionBuilder::create_dynamic_stack_slot`](https://docs.rs/cranelift-frontend/*/cranelift_frontend/struct.FunctionBuilder.html#method.create_dynamic_stack_slot)."] # [doc = ""] # [doc = " `StackSlot`s are most often used with"] # [doc = " [`stack_addr`](super::InstBuilder::stack_addr),"] # [doc = " [`stack_load`](super::InstBuilder::stack_load), and"] # [doc = " [`stack_store`](super::InstBuilder::stack_store)."] # [doc = ""] # [doc = " While the order is stable, it is arbitrary and does not necessarily resemble the stack order."] # [derive (Copy , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct StackSlot (u32) ;
};
}
