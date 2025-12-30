// Generated macro for ActorState (enum)
macro_rules! Depcrate_actorActorState {
() => {
// Module: crate::actor
// Provides: {"ActorState"}
// Dependencies: {}
# [doc = " Actor execution state"] # [derive (PartialEq , Debug , Copy , Clone)] pub enum ActorState { # [doc = " Actor is started."] Started , # [doc = " Actor is running."] Running , # [doc = " Actor is stopping."] Stopping , # [doc = " Actor is stopped."] Stopped , }
};
}
