// Generated macro for CFMessagePortContext (struct)
macro_rules! Depcrate_messageportCFMessagePortContext {
() => {
// Module: crate::messageport
// Provides: {"CFMessagePortContext"}
// Dependencies: {}
# [repr (C)] # [derive (Copy , Clone , Debug)] pub struct CFMessagePortContext { pub version : CFIndex , pub info : * mut c_void , pub retain : Option < unsafe extern "C" fn (info : * const c_void) -> * const c_void > , pub release : Option < unsafe extern "C" fn (info : * const c_void) > , pub copyDescription : Option < unsafe extern "C" fn (info : * const c_void) -> CFStringRef > , }
};
}
