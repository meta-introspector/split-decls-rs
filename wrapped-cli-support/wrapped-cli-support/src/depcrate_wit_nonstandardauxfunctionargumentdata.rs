// Generated macro for AuxFunctionArgumentData (struct)
macro_rules! Depcrate_wit_nonstandardAuxFunctionArgumentData {
() => {
// Module: crate::wit::nonstandard
// Provides: {"AuxFunctionArgumentData"}
// Dependencies: {}
# [doc = " Information about a functions' argument"] # [derive (Debug , Clone)] pub struct AuxFunctionArgumentData { # [doc = " Specifies the argument name"] pub name : String , # [doc = " Specifies the function argument type override"] pub ty_override : Option < String > , # [doc = " Specifies the argument description"] pub desc : Option < String > , }
};
}
