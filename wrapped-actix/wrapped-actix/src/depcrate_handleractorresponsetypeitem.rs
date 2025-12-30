// Generated macro for ActorResponseTypeItem (enum)
macro_rules! Depcrate_handlerActorResponseTypeItem {
() => {
// Module: crate::handler
// Provides: {"ActorResponseTypeItem"}
// Dependencies: {}
enum ActorResponseTypeItem < A , I > { Result (I) , Fut (Pin < Box < dyn ActorFuture < A , Output = I > > >) , }
};
}
