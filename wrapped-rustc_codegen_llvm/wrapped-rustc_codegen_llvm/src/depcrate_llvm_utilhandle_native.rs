// Generated macro for handle_native (function)
macro_rules! Depcrate_llvm_utilhandle_native {
() => {
// Module: crate::llvm_util
// Provides: {"handle_native"}
// Dependencies: {}
# [doc = " If the given string is `\"native\"`, returns the host CPU name according to"] # [doc = " LLVM. Otherwise, the string is returned as-is."] fn handle_native (cpu_name : & str) -> & str { match cpu_name { "native" => get_host_cpu_name () , _ => cpu_name , } }
};
}
