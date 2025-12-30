// Generated macro for Inner (struct)
macro_rules! DepcrateInner {
() => {
// Module: crate
// Provides: {"Inner"}
// Dependencies: {}
# [doc = " Inner state of the blocking executor."] struct Inner { # [doc = " Number of idle threads in the pool."] # [doc = ""] # [doc = " Idle threads are sleeping, waiting to get a task to run."] idle_count : usize , # [doc = " Total number of threads in the pool."] # [doc = ""] # [doc = " This is the number of idle threads + the number of active threads."] thread_count : usize , # [doc = " The queue of blocking tasks."] queue : VecDeque < Runnable > , # [doc = " Maximum number of threads in the pool"] thread_limit : Option < NonZeroUsize > , }
};
}
