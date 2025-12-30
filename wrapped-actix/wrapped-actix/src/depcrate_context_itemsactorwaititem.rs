// Generated macro for ActorWaitItem (struct)
macro_rules! Depcrate_context_itemsActorWaitItem {
() => {
// Module: crate::context_items
// Provides: {"ActorWaitItem"}
// Dependencies: {}
pub (crate) struct ActorWaitItem < A : Actor > (Pin < Box < dyn ActorFuture < A , Output = () > > >) ;
};
}
