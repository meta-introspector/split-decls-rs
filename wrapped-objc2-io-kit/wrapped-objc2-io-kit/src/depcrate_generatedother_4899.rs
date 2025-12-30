// Generated macro for other_4899 (other)
macro_rules! Depcrate_generatedother_4899 {
() => {
// Module: crate::generated
// Provides: {"other_4899"}
// Dependencies: {}
extern "C-unwind" { # [doc = " # Safety"] # [doc = ""] # [doc = " - `plugin_type` might not allow `None`."] # [doc = " - `interface_type` might not allow `None`."] # [doc = " - `the_interface` must be a valid pointer."] # [doc = " - `the_score` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IOCreatePlugInInterfaceForService (service : io_service_t , plugin_type : Option < & CFUUID > , interface_type : Option < & CFUUID > , the_interface : * mut * mut * mut IOCFPlugInInterface , the_score : * mut i32 ,) -> libc :: kern_return_t ; }
};
}
