// Generated macro for other_133 (other)
macro_rules! Depcrate_generatedother_133 {
() => {
// Module: crate::generated
// Provides: {"other_133"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Unregisters a registered callback function."] # [doc = ""] # [doc = " Parameter `session`: The session object."] # [doc = ""] # [doc = " Parameter `callback`: The registered callback function."] # [doc = ""] # [doc = " Parameter `context`: The user-defined context parameter."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `callback` must be a valid pointer."] # [doc = " - `context` must be a valid pointer or null."] # [cfg (feature = "DASession")] pub fn DAUnregisterCallback (session : & DASession , callback : NonNull < c_void > , context : * mut c_void ,) ; }
};
}
