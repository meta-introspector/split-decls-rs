// Generated macro for block_on (function)
macro_rules! Depcrate_executorblock_on {
() => {
// Module: crate::executor
// Provides: {"block_on"}
// Dependencies: {}
# [doc = " Runs the global and the local executor on the current thread"] # [doc = ""] # [doc = " Note: this calls `async_io::block_on` underneath."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let task = async_global_executor::spawn(async {"] # [doc = "     1 + 2"] # [doc = " });"] # [doc = " async_global_executor::block_on(async {"] # [doc = "     assert_eq!(task.await, 3);"] # [doc = " });"] # [doc = " ```"] pub fn block_on < F : Future < Output = T > , T > (future : F) -> T { LOCAL_EXECUTOR . with (| executor | crate :: reactor :: block_on (executor . run (future))) }
};
}
