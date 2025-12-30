// Generated macro for FunctionParameters (struct)
macro_rules! Depcrate_ir_functionFunctionParameters {
() => {
// Module: crate::ir::function
// Provides: {"FunctionParameters"}
// Dependencies: {}
# [doc = " Function parameters used when creating this function, and that will become applied after"] # [doc = " compilation to materialize the final `CompiledCode`."] # [derive (Clone , PartialEq)] # [cfg_attr (feature = "enable-serde" , derive (serde_derive :: Serialize , serde_derive :: Deserialize))] pub struct FunctionParameters { # [doc = " The first `SourceLoc` appearing in the function, serving as a base for every relative"] # [doc = " source loc in the function."] base_srcloc : Option < SourceLoc > , # [doc = " External user-defined function references."] user_named_funcs : PrimaryMap < UserExternalNameRef , UserExternalName > , # [doc = " Inverted mapping of `user_named_funcs`, to deduplicate internally."] user_ext_name_to_ref : HashMap < UserExternalName , UserExternalNameRef > , }
};
}
