// Generated macro for dbghelp (module)
macro_rules! Depcratedbghelp {
() => {
// Module: crate
// Provides: {"dbghelp"}
// Dependencies: {}
# [cfg (all (windows , any (target_env = "msvc" , all (target_env = "gnu" , any (target_arch = "x86" , target_arch = "arm"))) , not (target_vendor = "uwp")))] mod dbghelp ;
};
}
