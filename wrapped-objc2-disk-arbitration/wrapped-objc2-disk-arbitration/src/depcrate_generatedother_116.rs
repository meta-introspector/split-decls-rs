// Generated macro for other_116 (other)
macro_rules! Depcrate_generatedother_116 {
() => {
// Module: crate::generated
// Provides: {"other_116"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Registers a callback function to be called whenever a volume is to be mounted."] # [doc = ""] # [doc = " Parameter `session`: The session object."] # [doc = ""] # [doc = " Parameter `match`: The disk description keys to match.  Pass NULL for all disk objects."] # [doc = ""] # [doc = " Parameter `callback`: The callback function to call when a volume is to be mounted."] # [doc = ""] # [doc = " Parameter `context`: The user-defined context parameter to pass to the callback function."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `match` generic must be of the correct type."] # [doc = " - `match` generic must be of the correct type."] # [doc = " - `callback` must be implemented correctly."] # [doc = " - `context` must be a valid pointer or null."] # [cfg (all (feature = "DADisk" , feature = "DADissenter" , feature = "DASession"))] pub fn DARegisterDiskMountApprovalCallback (session : & DASession , r#match : Option < & CFDictionary > , callback : DADiskMountApprovalCallback , context : * mut c_void ,) ; }
};
}
