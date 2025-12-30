// Generated macro for prepare_thin (function)
macro_rules! Depcrate_back_ltoprepare_thin {
() => {
// Module: crate::back::lto
// Provides: {"prepare_thin"}
// Dependencies: {}
pub (crate) fn prepare_thin (module : ModuleCodegen < GccContext >) -> (String , ThinBuffer) { let name = module . name ; let buffer = ThinBuffer :: new (& module . module_llvm . context) ; (name , buffer) }
};
}
