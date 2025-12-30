// Generated macro for impl_714 (impl)
macro_rules! Depcrate_ir_extfuncimpl_714 {
() => {
// Module: crate::ir::extfunc
// Provides: {"impl_714"}
// Dependencies: {}
impl ExtFuncData { # [doc = " Returns a displayable version of the `ExtFuncData`, with or without extra context to"] # [doc = " prettify the output."] pub fn display < 'a > (& 'a self , params : Option < & 'a FunctionParameters > ,) -> DisplayableExtFuncData < 'a > { DisplayableExtFuncData { ext_func : self , params , } } }
};
}
