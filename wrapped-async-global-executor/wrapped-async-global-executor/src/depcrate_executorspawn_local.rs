// Generated macro for spawn_local (function)
macro_rules! Depcrate_executorspawn_local {
() => {
// Module: crate::executor
// Provides: {"spawn_local"}
// Dependencies: {}
# [doc = " Spawns a task onto the local executor."] # [doc = ""] # [doc = ""] # [doc = " The task does not need to be `Send` as it will be spawned on the same thread."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use futures_lite::future;"] # [doc = ""] # [doc = " let task1 = async_global_executor::spawn_local(async {"] # [doc = "     1 + 2"] # [doc = " });"] # [doc = " let task2 = async_global_executor::spawn_local(async {"] # [doc = "     3 + 4"] # [doc = " });"] # [doc = " let task = future::zip(task1, task2);"] # [doc = ""] # [doc = " async_global_executor::block_on(async {"] # [doc = "     assert_eq!(task.await, (3, 7));"] # [doc = " });"] # [doc = " ```"] pub fn spawn_local < F : Future < Output = T > + 'static , T : 'static > (future : F) -> Task < T > { LOCAL_EXECUTOR . with (| executor | executor . spawn (future)) }
};
}
