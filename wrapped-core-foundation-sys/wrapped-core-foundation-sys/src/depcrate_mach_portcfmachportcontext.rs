// Generated macro for CFMachPortContext (struct)
macro_rules! Depcrate_mach_portCFMachPortContext {
() => {
// Module: crate::mach_port
// Provides: {"CFMachPortContext"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy)] pub struct CFMachPortContext { pub version : CFIndex , pub info : * mut c_void , pub retain : extern "C" fn (info : * const c_void) -> * const c_void , pub release : extern "C" fn (info : * const c_void) , pub copyDescription : extern "C" fn (info : * const c_void) -> CFStringRef , }
};
}
