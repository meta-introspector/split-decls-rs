// Generated macro for find_funcref (function)
macro_rules! Depcrate_ir_libcallfind_funcref {
() => {
// Module: crate::ir::libcall
// Provides: {"find_funcref"}
// Dependencies: {}
# [doc = " Get the existing function reference for `libcall` in `func` if it exists."] fn find_funcref (libcall : LibCall , func : & Function) -> Option < FuncRef > { for (fref , func_data) in func . dfg . ext_funcs . iter () . rev () { match func_data . name { ExternalName :: LibCall (lc) => { if lc == libcall { return Some (fref) ; } } _ => break , } } None }
};
}
