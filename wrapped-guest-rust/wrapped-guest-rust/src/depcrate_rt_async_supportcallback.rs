// Generated macro for callback (function)
macro_rules! Depcrate_rt_async_supportcallback {
() => {
// Module: crate::rt::async_support
// Provides: {"callback"}
// Dependencies: {}
# [doc = " Handle a progress notification from the host regarding either a call to an"] # [doc = " async-lowered import or a stream/future read/write operation."] # [doc = ""] # [doc = " # Unsafety"] # [doc = ""] # [doc = " This function assumes that `context_get()` returns a `FutureState`."] # [doc (hidden)] pub unsafe fn callback (event0 : u32 , event1 : u32 , event2 : u32) -> u32 { let state = context_get () . cast :: < FutureState < 'static > > () ; assert ! (! state . is_null ()) ; unsafe { context_set (ptr :: null_mut ()) ; } unsafe { let (rc , done) = (* state) . callback (event0 , event1 , event2) ; if done { drop (Box :: from_raw (state)) ; } else { context_set (state . cast ()) ; } rtdebug ! (" => (cb) {rc:#x}") ; rc } }
};
}
