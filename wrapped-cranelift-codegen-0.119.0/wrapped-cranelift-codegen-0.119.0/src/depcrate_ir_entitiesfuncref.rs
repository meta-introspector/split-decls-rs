// Generated macro for FuncRef (struct)
macro_rules! Depcrate_ir_entitiesFuncRef {
() => {
// Module: crate::ir::entities
// Provides: {"FuncRef"}
// Dependencies: {}
# [doc = " An opaque reference to another [`Function`](super::Function)."] # [doc = ""] # [doc = " `FuncRef`s are used for [direct](super::InstBuilder::call) function calls"] # [doc = " and by [`func_addr`](super::InstBuilder::func_addr) for use in"] # [doc = " [indirect](super::InstBuilder::call_indirect) function calls."] # [doc = ""] # [doc = " `FuncRef`s can be created with"] # [doc = ""] # [doc = " - [`FunctionBuilder::import_function`](https://docs.rs/cranelift-frontend/*/cranelift_frontend/struct.FunctionBuilder.html#method.import_function)"] # [doc = "   for external functions"] # [doc = " - [`Module::declare_func_in_func`](https://docs.rs/cranelift-module/*/cranelift_module/trait.Module.html#method.declare_func_in_func)"] # [doc = "   for functions declared elsewhere in the same native"] # [doc = "   [`Module`](https://docs.rs/cranelift-module/*/cranelift_module/trait.Module.html)"] # [doc = ""] # [doc = " While the order is stable, it is arbitrary."] # [derive (Copy , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct FuncRef (u32) ;
};
}
