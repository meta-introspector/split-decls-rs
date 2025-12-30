// Generated macro for spawn (function)
macro_rules! Depcrate_executorspawn {
() => {
// Module: crate::executor
// Provides: {"spawn"}
// Dependencies: {}
# [doc = " Spawns a task onto the multi-threaded global executor."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use futures_lite::future;"] # [doc = ""] # [doc = " let task1 = async_global_executor::spawn(async {"] # [doc = "     1 + 2"] # [doc = " });"] # [doc = " let task2 = async_global_executor::spawn(async {"] # [doc = "     3 + 4"] # [doc = " });"] # [doc = " let task = future::zip(task1, task2);"] # [doc = ""] # [doc = " async_global_executor::block_on(async {"] # [doc = "     assert_eq!(task.await, (3, 7));"] # [doc = " });"] # [doc = " ```"] pub fn spawn < F : Future < Output = T > + Send + 'static , T : Send + 'static > (future : F) -> Task < T > { crate :: init () ; GLOBAL_EXECUTOR . spawn (future) }
};
}
