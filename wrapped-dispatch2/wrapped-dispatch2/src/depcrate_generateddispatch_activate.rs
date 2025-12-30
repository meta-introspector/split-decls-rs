// Generated macro for dispatch_activate (function)
macro_rules! Depcrate_generateddispatch_activate {
() => {
// Module: crate::generated
// Provides: {"dispatch_activate"}
// Dependencies: {}
# [doc = " Activates the specified dispatch object."] # [doc = ""] # [doc = ""] # [doc = " Dispatch objects such as queues and sources may be created in an inactive"] # [doc = " state. Objects in this state have to be activated before any blocks"] # [doc = " associated with them will be invoked."] # [doc = ""] # [doc = " The target queue of inactive objects can be changed using"] # [doc = " dispatch_set_target_queue(). Change of target queue is no longer permitted"] # [doc = " once an initially inactive object has been activated."] # [doc = ""] # [doc = " Calling dispatch_activate() on an active object has no effect."] # [doc = " Releasing the last reference count on an inactive object is undefined."] # [doc = ""] # [doc = ""] # [doc = " Parameter `object`: The object to be activated."] # [doc = " The result of passing NULL in this parameter is undefined."] # [inline] pub extern "C" fn dispatch_activate (object : NonNull < dispatch_object_s >) { extern "C" { fn dispatch_activate (object : NonNull < dispatch_object_s >) ; } unsafe { dispatch_activate (object) } }
};
}
