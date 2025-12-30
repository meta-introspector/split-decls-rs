// Generated macro for impl_50 (impl)
macro_rules! Depcrate_build_dependenciesimpl_50 {
() => {
// Module: crate::build_dependencies
// Provides: {"impl_50"}
// Dependencies: {}
impl BuildScriptOutput { fn is_empty (& self) -> bool { self . cfgs . is_empty () && self . envs . is_empty () && self . out_dir . is_none () && matches ! (self . proc_macro_dylib_path , ProcMacroDylibPath :: NotBuilt | ProcMacroDylibPath :: NotProcMacro) } }
};
}
