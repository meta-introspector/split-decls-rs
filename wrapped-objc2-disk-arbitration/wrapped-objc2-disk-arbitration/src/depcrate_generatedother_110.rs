// Generated macro for other_110 (other)
macro_rules! Depcrate_generatedother_110 {
() => {
// Module: crate::generated
// Provides: {"other_110"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Registers a callback function to be called whenever a disk description has changed."] # [doc = ""] # [doc = " Parameter `session`: The session object."] # [doc = ""] # [doc = " Parameter `match`: The disk description keys to match.  Pass NULL for all disk objects."] # [doc = ""] # [doc = " Parameter `watch`: The disk description keys to watch.  Pass NULL for all keys."] # [doc = ""] # [doc = " Parameter `callback`: The callback function to call when a watched key changes."] # [doc = ""] # [doc = " Parameter `context`: The user-defined context parameter to pass to the callback function."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `match` generic must be of the correct type."] # [doc = " - `match` generic must be of the correct type."] # [doc = " - `watch` generic must be of the correct type."] # [doc = " - `callback` must be implemented correctly."] # [doc = " - `context` must be a valid pointer or null."] # [cfg (all (feature = "DADisk" , feature = "DASession"))] pub fn DARegisterDiskDescriptionChangedCallback (session : & DASession , r#match : Option < & CFDictionary > , watch : Option < & CFArray > , callback : DADiskDescriptionChangedCallback , context : * mut c_void ,) ; }
};
}
