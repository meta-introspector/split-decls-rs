// Generated macro for find_tool_with_env (function)
macro_rules! Depcrate_find_toolsfind_tool_with_env {
() => {
// Module: crate::find_tools
// Provides: {"find_tool_with_env"}
// Dependencies: {}
pub fn find_tool_with_env (full_arch : & str , tool : & str , env_getter : & dyn EnvGetter) -> Option < Tool > { let target = TargetArch :: new (full_arch) ? ; if tool . contains ("msbuild") { return impl_ :: find_msbuild (target , env_getter) ; } if tool . contains ("devenv") { return impl_ :: find_devenv (target , env_getter) ; } if ["clang" , "lldb" , "llvm" , "ld" , "lld"] . iter () . any (| & t | tool . contains (t)) { return impl_ :: find_llvm_tool (tool , target , env_getter) ; } impl_ :: find_msvc_environment (tool , target , env_getter) . or_else (| | impl_ :: find_msvc_15plus (tool , target , env_getter)) . or_else (| | impl_ :: find_msvc_14 (tool , target , env_getter)) }
};
}
