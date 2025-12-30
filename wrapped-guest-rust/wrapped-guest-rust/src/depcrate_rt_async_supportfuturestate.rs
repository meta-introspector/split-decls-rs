// Generated macro for FutureState (struct)
macro_rules! Depcrate_rt_async_supportFutureState {
() => {
// Module: crate::rt::async_support
// Provides: {"FutureState"}
// Dependencies: {}
# [doc = " Represents a task created by either a call to an async-lifted export or a"] # [doc = " future run using `block_on` or `start_task`."] struct FutureState < 'a > { # [doc = " Remaining work to do (if any) before this task can be considered \"done\"."] # [doc = ""] # [doc = " Note that we won't tell the host the task is done until this is drained"] # [doc = " and `waitables` is empty."] tasks : spawn :: Tasks < 'a > , # [doc = " The waitable set containing waitables created by this task, if any."] waitable_set : Option < WaitableSet > , # [doc = " State of all waitables in `waitable_set`, and the ptr/callback they're"] # [doc = " associated with."] waitables : BTreeMap < u32 , (* mut c_void , unsafe extern "C" fn (* mut c_void , u32)) > , # [doc = " Raw structure used to pass to `cabi::wasip3_task_set`"] wasip3_task : cabi :: wasip3_task , # [doc = " Rust-level state for the waker, notably a bool as to whether this has"] # [doc = " been woken."] waker : Arc < FutureWaker > , # [doc = " Clone of `waker` field, but represented as `std::task::Waker`."] waker_clone : Waker , }
};
}
