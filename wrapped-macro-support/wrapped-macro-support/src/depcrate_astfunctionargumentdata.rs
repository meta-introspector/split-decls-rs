// Generated macro for FunctionArgumentData (struct)
macro_rules! Depcrate_astFunctionArgumentData {
() => {
// Module: crate::ast
// Provides: {"FunctionArgumentData"}
// Dependencies: {}
# [doc = " Information about a function's argument"] # [cfg_attr (feature = "extra-traits" , derive (Debug))] # [derive (Clone)] pub struct FunctionArgumentData { # [doc = " Specifies the type of the function's argument"] pub pat_type : syn :: PatType , # [doc = " Specifies the JS argument name override"] pub js_name : Option < String > , # [doc = " Specifies the JS function argument type override"] pub js_type : Option < String > , # [doc = " Specifies the argument description"] pub desc : Option < String > , }
};
}
