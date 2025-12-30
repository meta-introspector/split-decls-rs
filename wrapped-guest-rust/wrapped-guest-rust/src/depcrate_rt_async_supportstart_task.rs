// Generated macro for start_task (function)
macro_rules! Depcrate_rt_async_supportstart_task {
() => {
// Module: crate::rt::async_support
// Provides: {"start_task"}
// Dependencies: {}
# [doc = " Starts execution of the `task` provided, an asynchronous computation."] # [doc = ""] # [doc = " This is used for async-lifted exports at their definition site. The"] # [doc = " representation of the export is `task` and this function is called from the"] # [doc = " entrypoint. The code returned here is the same as the callback associated"] # [doc = " with this export, and the callback will be used if this task doesn't exit"] # [doc = " immediately with its result."] # [doc (hidden)] pub fn start_task (task : impl Future < Output = () > + 'static) -> i32 { let state = Box :: into_raw (Box :: new (FutureState :: new (Box :: pin (task)))) ; unsafe { assert ! (context_get () . is_null ()) ; context_set (state . cast ()) ; callback (EVENT_NONE , 0 , 0) as i32 } }
};
}
