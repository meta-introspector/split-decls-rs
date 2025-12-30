// Generated macro for CFFileDescriptorContext (struct)
macro_rules! Depcrate_filedescriptorCFFileDescriptorContext {
() => {
// Module: crate::filedescriptor
// Provides: {"CFFileDescriptorContext"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Clone , Copy)] pub struct CFFileDescriptorContext { pub version : CFIndex , pub info : * mut c_void , pub retain : Option < extern "C" fn (info : * const c_void) -> * const c_void > , pub release : Option < extern "C" fn (info : * const c_void) > , pub copyDescription : Option < extern "C" fn (info : * const c_void) -> CFStringRef > , }
};
}
