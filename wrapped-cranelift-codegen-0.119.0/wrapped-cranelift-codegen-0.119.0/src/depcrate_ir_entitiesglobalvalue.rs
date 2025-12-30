// Generated macro for GlobalValue (struct)
macro_rules! Depcrate_ir_entitiesGlobalValue {
() => {
// Module: crate::ir::entities
// Provides: {"GlobalValue"}
// Dependencies: {}
# [doc = " An opaque reference to a global value."] # [doc = ""] # [doc = " A `GlobalValue` is a [`Value`] that will be live across the entire"] # [doc = " function lifetime. It can be preloaded from other global values."] # [doc = ""] # [doc = " You can create a `GlobalValue` in the following ways:"] # [doc = ""] # [doc = " - When compiling to native code, you can use it for objects in static memory with"] # [doc = "   [`Module::declare_data_in_func`](https://docs.rs/cranelift-module/*/cranelift_module/trait.Module.html#method.declare_data_in_func)."] # [doc = " - For any compilation target, it can be registered with"] # [doc = "   [`FunctionBuilder::create_global_value`](https://docs.rs/cranelift-frontend/*/cranelift_frontend/struct.FunctionBuilder.html#method.create_global_value)."] # [doc = ""] # [doc = " `GlobalValue`s can be retrieved with"] # [doc = " [`InstBuilder:global_value`](super::InstBuilder::global_value)."] # [doc = ""] # [doc = " While the order is stable, it is arbitrary."] # [derive (Copy , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct GlobalValue (u32) ;
};
}
