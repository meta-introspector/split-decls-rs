// Generated macro for StaticExecutor (struct)
macro_rules! Depcrate_static_executorsStaticExecutor {
() => {
// Module: crate::static_executors
// Provides: {"StaticExecutor"}
// Dependencies: {}
# [doc = " A static-lifetimed async [`Executor`]."] # [doc = ""] # [doc = " This is primarily intended to be used in [`static`] variables, or types intended to be used, or can be created in non-static"] # [doc = " contexts via [`Executor::leak`]."] # [doc = ""] # [doc = " Spawning, running, and finishing tasks are optimized with the assumption that the executor will never be `Drop`'ed."] # [doc = " A static executor may require signficantly less overhead in both single-threaded and mulitthreaded use cases."] # [doc = ""] # [doc = " As this type does not implement `Drop`, losing the handle to the executor or failing"] # [doc = " to consistently drive the executor with [`StaticExecutor::tick`] or"] # [doc = " [`StaticExecutor::run`] will cause the all spawned tasks to permanently leak. Any"] # [doc = " tasks at the time will not be cancelled."] # [doc = ""] # [doc = " [`static`]: https://doc.rust-lang.org/std/keyword.static.html"] # [repr (transparent)] pub struct StaticExecutor { state : State , }
};
}
