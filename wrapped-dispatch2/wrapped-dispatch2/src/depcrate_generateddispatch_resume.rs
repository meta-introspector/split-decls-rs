// Generated macro for dispatch_resume (function)
macro_rules! Depcrate_generateddispatch_resume {
() => {
// Module: crate::generated
// Provides: {"dispatch_resume"}
// Dependencies: {}
# [doc = " Resumes the invocation of blocks on a dispatch object."] # [doc = ""] # [doc = ""] # [doc = " Dispatch objects can be suspended with dispatch_suspend(), which increments"] # [doc = " an internal suspension count. dispatch_resume() is the inverse operation,"] # [doc = " and consumes suspension counts. When the last suspension count is consumed,"] # [doc = " blocks associated with the object will be invoked again."] # [doc = ""] # [doc = " For backward compatibility reasons, dispatch_resume() on an inactive and not"] # [doc = " otherwise suspended dispatch source object has the same effect as calling"] # [doc = " dispatch_activate(). For new code, using dispatch_activate() is preferred."] # [doc = ""] # [doc = " If the specified object has zero suspension count and is not an inactive"] # [doc = " source, this function will result in an assertion and the process being"] # [doc = " terminated."] # [doc = ""] # [doc = ""] # [doc = " Parameter `object`: The object to be resumed."] # [doc = " The result of passing NULL in this parameter is undefined."] # [inline] pub extern "C" fn dispatch_resume (object : NonNull < dispatch_object_s >) { extern "C" { fn dispatch_resume (object : NonNull < dispatch_object_s >) ; } unsafe { dispatch_resume (object) } }
};
}
