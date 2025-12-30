// Generated macro for CFMachPortInvalidationCallBack (type)
macro_rules! Depcrate_mach_portCFMachPortInvalidationCallBack {
() => {
// Module: crate::mach_port
// Provides: {"CFMachPortInvalidationCallBack"}
// Dependencies: {}
pub type CFMachPortInvalidationCallBack = extern "C" fn (port : CFMachPortRef , info : * mut c_void) ;
};
}
