// Generated macro for get_host_cpu_name (function)
macro_rules! Depcrate_llvm_utilget_host_cpu_name {
() => {
// Module: crate::llvm_util
// Provides: {"get_host_cpu_name"}
// Dependencies: {}
# [doc = " Returns the host CPU name, according to LLVM."] fn get_host_cpu_name () -> & 'static str { let mut len = 0 ; let slice : & 'static [u8] = unsafe { let ptr = llvm :: LLVMRustGetHostCPUName (& mut len) ; assert ! (! ptr . is_null ()) ; slice :: from_raw_parts (ptr , len) } ; str :: from_utf8 (slice) . expect ("host CPU name should be UTF-8") }
};
}
