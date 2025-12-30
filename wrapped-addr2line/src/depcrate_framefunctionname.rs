// Generated macro for FunctionName (struct)
macro_rules! Depcrate_frameFunctionName {
() => {
// Module: crate::frame
// Provides: {"FunctionName"}
// Dependencies: {}
# [doc = " A function name."] pub struct FunctionName < R : gimli :: Reader > { # [doc = " The name of the function."] pub name : R , # [doc = " The language of the compilation unit containing this function."] pub language : Option < gimli :: DwLang > , }
};
}
