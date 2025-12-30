// Generated macro for FunctionReturnData (struct)
macro_rules! Depcrate_astFunctionReturnData {
() => {
// Module: crate::ast
// Provides: {"FunctionReturnData"}
// Dependencies: {}
# [doc = " Information about a function's return"] # [cfg_attr (feature = "extra-traits" , derive (Debug))] # [derive (Clone)] pub struct FunctionReturnData { # [doc = " Specifies the type of the function's return"] pub r#type : syn :: Type , # [doc = " Specifies the JS return type override"] pub js_type : Option < String > , # [doc = " Specifies the return description"] pub desc : Option < String > , }
};
}
