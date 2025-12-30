// Generated macro for other_26 (other)
macro_rules! Depcrate_generatedother_26 {
() => {
// Module: crate::generated
// Provides: {"other_26"}
// Dependencies: {}
extern "C" { # [doc = " Decrement the reference count of a dispatch object."] # [doc = ""] # [doc = ""] # [doc = " A dispatch object is asynchronously deallocated once all references are"] # [doc = " released (i.e. the reference count becomes zero). The system does not"] # [doc = " guarantee that a given client is the last or only reference to a given"] # [doc = " object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `object`: The object to release."] # [doc = " The result of passing NULL in this parameter is undefined."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `object` must be a valid pointer."] pub fn dispatch_release (object : NonNull < dispatch_object_s >) ; }
};
}
