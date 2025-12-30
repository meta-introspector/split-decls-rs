// Generated macro for Function (struct)
macro_rules! Depcrate_ir_functionFunction {
() => {
// Module: crate::ir::function
// Provides: {"Function"}
// Dependencies: {}
# [doc = " Functions can be cloned, but it is not a very fast operation."] # [doc = " The clone will have all the same entity numbers as the original."] # [derive (Clone , PartialEq)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct Function { # [doc = " Name of this function."] # [doc = ""] # [doc = " Mostly used by `.clif` files, only there for debugging / naming purposes."] pub name : UserFuncName , # [doc = " All the fields required for compiling a function, independently of details irrelevant to"] # [doc = " compilation and that are stored in the `FunctionParameters` `params` field instead."] pub stencil : FunctionStencil , # [doc = " All the parameters that can be applied onto the function stencil, that is, that don't"] # [doc = " matter when caching compilation artifacts."] pub params : FunctionParameters , }
};
}
