// Generated macro for other_29 (other)
macro_rules! Depcrate_generatedother_29 {
() => {
// Module: crate::generated
// Provides: {"other_29"}
// Dependencies: {}
extern "C" { # [doc = " Set the finalizer function for a dispatch object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `object`: The dispatch object to modify."] # [doc = " The result of passing NULL in this parameter is undefined."] # [doc = ""] # [doc = ""] # [doc = " Parameter `finalizer`: The finalizer function pointer."] # [doc = ""] # [doc = ""] # [doc = " A dispatch object's finalizer will be invoked on the object's target queue"] # [doc = " after all references to the object have been released. This finalizer may be"] # [doc = " used by the application to release any resources associated with the object,"] # [doc = " such as freeing the object's context."] # [doc = " The context parameter passed to the finalizer function is the current"] # [doc = " context of the dispatch object at the time the finalizer call is made."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `object` must be a valid pointer."] # [doc = " - `finalizer` must be implemented correctly."] pub fn dispatch_set_finalizer_f (object : NonNull < dispatch_object_s > , finalizer : dispatch_function_t ,) ; }
};
}
