// Generated macro for FallibleTask (struct)
macro_rules! Depcrate_taskFallibleTask {
() => {
// Module: crate::task
// Provides: {"FallibleTask"}
// Dependencies: {}
# [doc = " A spawned task with a fallible response."] # [doc = ""] # [doc = " This type behaves like [`Task`], however it produces an `Option<T>` when"] # [doc = " polled and will return `None` if the executor dropped its"] # [doc = " [`Runnable`][`super::Runnable`] without being run."] # [doc = ""] # [doc = " This can be useful to avoid the panic produced when polling the `Task`"] # [doc = " future if the executor dropped its `Runnable`."] # [must_use = "tasks get canceled when dropped, use `.detach()` to run them in the background"] pub struct FallibleTask < T , M = () > { task : Task < T , M > , }
};
}
