// Generated macro for other_134 (other)
macro_rules! Depcrate_generatedother_134 {
() => {
// Module: crate::generated
// Provides: {"other_134"}
// Dependencies: {}
extern "C-unwind" { # [doc = " # Safety"] # [doc = ""] # [doc = " - `callback` must be a valid pointer."] # [doc = " - `context` must be a valid pointer or null."] # [cfg (feature = "DASession")] pub fn DAUnregisterApprovalCallback (session : & DASession , callback : NonNull < c_void > , context : * mut c_void ,) ; }
};
}
