// Generated macro for CFMachPortCallBack (type)
macro_rules! Depcrate_mach_portCFMachPortCallBack {
() => {
// Module: crate::mach_port
// Provides: {"CFMachPortCallBack"}
// Dependencies: {}
pub type CFMachPortCallBack = extern "C" fn (port : CFMachPortRef , msg : * mut c_void , size : CFIndex , info : * mut c_void) ;
};
}
