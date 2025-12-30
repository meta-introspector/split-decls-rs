// Generated macro for other_25 (other)
macro_rules! Depcrate_generatedother_25 {
() => {
// Module: crate::generated
// Provides: {"other_25"}
// Dependencies: {}
extern "C" { # [doc = " Increment the reference count of a dispatch object."] # [doc = ""] # [doc = ""] # [doc = " Calls to dispatch_retain() must be balanced with calls to"] # [doc = " dispatch_release()."] # [doc = ""] # [doc = ""] # [doc = " Parameter `object`: The object to retain."] # [doc = " The result of passing NULL in this parameter is undefined."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `object` must be a valid pointer."] pub fn dispatch_retain (object : NonNull < dispatch_object_s >) ; }
};
}
