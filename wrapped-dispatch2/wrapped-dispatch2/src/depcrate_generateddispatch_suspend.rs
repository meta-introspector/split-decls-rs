// Generated macro for dispatch_suspend (function)
macro_rules! Depcrate_generateddispatch_suspend {
() => {
// Module: crate::generated
// Provides: {"dispatch_suspend"}
// Dependencies: {}
# [doc = " Suspends the invocation of blocks on a dispatch object."] # [doc = ""] # [doc = ""] # [doc = " A suspended object will not invoke any blocks associated with it. The"] # [doc = " suspension of an object will occur after any running block associated with"] # [doc = " the object completes."] # [doc = ""] # [doc = " Calls to dispatch_suspend() must be balanced with calls"] # [doc = " to dispatch_resume()."] # [doc = ""] # [doc = ""] # [doc = " Parameter `object`: The object to be suspended."] # [doc = " The result of passing NULL in this parameter is undefined."] # [inline] pub extern "C" fn dispatch_suspend (object : NonNull < dispatch_object_s >) { extern "C" { fn dispatch_suspend (object : NonNull < dispatch_object_s >) ; } unsafe { dispatch_suspend (object) } }
};
}
