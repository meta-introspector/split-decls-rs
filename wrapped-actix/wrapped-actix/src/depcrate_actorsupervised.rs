// Generated macro for Supervised (trait)
macro_rules! Depcrate_actorSupervised {
() => {
// Module: crate::actor
// Provides: {"Supervised"}
// Dependencies: {}
# [allow (unused_variables)] # [doc = " Actors with the ability to restart after failure."] # [doc = ""] # [doc = " Supervised actors can be managed by a"] # [doc = " [`Supervisor`](struct.Supervisor.html). As an additional lifecycle"] # [doc = " event, the `restarting` method can be implemented."] # [doc = ""] # [doc = " If a supervised actor fails, its supervisor creates new execution"] # [doc = " context and restarts the actor, invoking its `restarting` method."] # [doc = " After a call to this method, the actor's execution state changes"] # [doc = " to `Started` and the regular lifecycle process starts."] # [doc = ""] # [doc = " The `restarting` method gets called with the newly constructed"] # [doc = " `Context` object."] pub trait Supervised : Actor { # [doc = " Called when the supervisor restarts a failed actor."] fn restarting (& mut self , ctx : & mut < Self as Actor > :: Context) { } }
};
}
