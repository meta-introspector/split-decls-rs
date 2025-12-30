// Generated macro for ThreadLocal (struct)
macro_rules! Depcrate_threadThreadLocal {
() => {
// Module: crate::thread
// Provides: {"ThreadLocal"}
// Dependencies: {}
# [doc = " A thread-local pointer."] # [doc = ""] # [doc = " It is neither `Sync` nor `Send`."] # [repr (transparent)] # [derive (Copy , Clone)] pub struct ThreadLocal < T > (* const T) ;
};
}
