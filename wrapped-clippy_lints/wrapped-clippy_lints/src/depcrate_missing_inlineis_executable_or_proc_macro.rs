// Generated macro for is_executable_or_proc_macro (function)
macro_rules! Depcrate_missing_inlineis_executable_or_proc_macro {
() => {
// Module: crate::missing_inline
// Provides: {"is_executable_or_proc_macro"}
// Dependencies: {}
fn is_executable_or_proc_macro (cx : & LateContext < '_ >) -> bool { use rustc_session :: config :: CrateType ; cx . tcx . crate_types () . iter () . any (| t : & CrateType | matches ! (t , CrateType :: Executable | CrateType :: ProcMacro)) }
};
}
