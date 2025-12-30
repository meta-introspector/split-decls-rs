// Generated macro for StaticLocalExecutor (struct)
macro_rules! Depcrate_static_executorsStaticLocalExecutor {
() => {
// Module: crate::static_executors
// Provides: {"StaticLocalExecutor"}
// Dependencies: {}
# [doc = " A static async [`LocalExecutor`] created from [`LocalExecutor::leak`]."] # [doc = ""] # [doc = " This is primarily intended to be used in [`thread_local`] variables, or can be created in non-static"] # [doc = " contexts via [`LocalExecutor::leak`]."] # [doc = ""] # [doc = " Spawning, running, and finishing tasks are optimized with the assumption that the executor will never be `Drop`'ed."] # [doc = " A static executor may require signficantly less overhead in both single-threaded and mulitthreaded use cases."] # [doc = ""] # [doc = " As this type does not implement `Drop`, losing the handle to the executor or failing"] # [doc = " to consistently drive the executor with [`StaticLocalExecutor::tick`] or"] # [doc = " [`StaticLocalExecutor::run`] will cause the all spawned tasks to permanently leak. Any"] # [doc = " tasks at the time will not be cancelled."] # [doc = ""] # [doc = " [`thread_local]: https://doc.rust-lang.org/std/macro.thread_local.html"] # [repr (transparent)] pub struct StaticLocalExecutor { state : State , marker_ : PhantomData < UnsafeCell < () > > , }
};
}
