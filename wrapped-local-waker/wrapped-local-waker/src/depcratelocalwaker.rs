// Generated macro for LocalWaker (struct)
macro_rules! DepcrateLocalWaker {
() => {
// Module: crate
// Provides: {"LocalWaker"}
// Dependencies: {}
# [doc = " A synchronization primitive for task wakeup."] # [doc = ""] # [doc = " Sometimes the task interested in a given event will change over time. A `LocalWaker` can"] # [doc = " coordinate concurrent notifications with the consumer, potentially \"updating\" the underlying"] # [doc = " task to wake up. This is useful in scenarios where a computation completes in another task and"] # [doc = " wants to notify the consumer, but the consumer is in the process of being migrated to a new"] # [doc = " logical task."] # [doc = ""] # [doc = " Consumers should call [`register`] before checking the result of a computation and producers"] # [doc = " should call [`wake`] after producing the computation (this differs from the usual `thread::park`"] # [doc = " pattern). It is also permitted for [`wake`] to be called _before_ [`register`]. This results in"] # [doc = " a no-op."] # [doc = ""] # [doc = " A single `LocalWaker` may be reused for any number of calls to [`register`] or [`wake`]."] # [doc = ""] # [doc = " [`register`]: LocalWaker::register"] # [doc = " [`wake`]: LocalWaker::wake"] # [derive (Default)] pub struct LocalWaker { pub (crate) waker : Cell < Option < Waker > > , _phantom : PhantomData < * const () > , }
};
}
