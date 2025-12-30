// Generated macro for get_probestack_funcref (function)
macro_rules! Depcrate_ir_libcallget_probestack_funcref {
() => {
// Module: crate::ir::libcall
// Provides: {"get_probestack_funcref"}
// Dependencies: {}
# [doc = " Get a function reference for the probestack function in `func`."] # [doc = ""] # [doc = " If there is an existing reference, use it, otherwise make a new one."] pub fn get_probestack_funcref (func : & mut Function) -> Option < FuncRef > { find_funcref (LibCall :: Probestack , func) }
};
}
