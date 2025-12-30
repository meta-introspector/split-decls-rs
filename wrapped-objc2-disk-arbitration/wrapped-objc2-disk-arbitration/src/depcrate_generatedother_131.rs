// Generated macro for other_131 (other)
macro_rules! Depcrate_generatedother_131 {
() => {
// Module: crate::generated
// Provides: {"other_131"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Registers a callback function to be called whenever a disk has been probed."] # [doc = ""] # [doc = " Parameter `session`: The session object."] # [doc = ""] # [doc = " Parameter `match`: The disk description keys to match.  Pass NULL for all disk objects."] # [doc = ""] # [doc = " Parameter `order`: The callback order, from lowest to highest.  Pass 0 for the default."] # [doc = ""] # [doc = " Parameter `callback`: The callback function to call when a disk has been probed."] # [doc = ""] # [doc = " Parameter `context`: The user-defined context parameter to pass to the callback function."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `match` generic must be of the correct type."] # [doc = " - `match` generic must be of the correct type."] # [doc = " - `callback` must be implemented correctly."] # [doc = " - `context` must be a valid pointer or null."] # [cfg (all (feature = "DADisk" , feature = "DASession"))] pub fn DARegisterDiskPeekCallback (session : & DASession , r#match : Option < & CFDictionary > , order : CFIndex , callback : DADiskPeekCallback , context : * mut c_void ,) ; }
};
}
