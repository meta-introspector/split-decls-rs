// Generated macro for LocalBoxActorFuture (type)
macro_rules! Depcrate_fut_futureLocalBoxActorFuture {
() => {
// Module: crate::fut::future
// Provides: {"LocalBoxActorFuture"}
// Dependencies: {}
# [doc = " Type alias for a pinned box [`ActorFuture`] trait object."] pub type LocalBoxActorFuture < A , I > = Pin < Box < dyn ActorFuture < A , Output = I > > > ;
};
}
