// Generated macro for ActorContext (trait)
macro_rules! Depcrate_actorActorContext {
() => {
// Module: crate::actor
// Provides: {"ActorContext"}
// Dependencies: {}
# [doc = " Actor execution context."] # [doc = ""] # [doc = " Each actor runs within a specific execution context. The actor's"] # [doc = " associated type `Actor::Context` defines the context to use for"] # [doc = " the actor, and must implement the `ActorContext` trait."] # [doc = ""] # [doc = " The execution context defines the type of execution, and the"] # [doc = " actor's communication channels (message handling)."] pub trait ActorContext : Sized { # [doc = " Immediately stop processing incoming messages and switch to a"] # [doc = " `stopping` state. This only affects actors that are currently"] # [doc = " `running`. Future attempts to queue messages will fail."] fn stop (& mut self) ; # [doc = " Terminate actor execution unconditionally. This sets the actor"] # [doc = " into the `stopped` state. This causes future attempts to queue"] # [doc = " messages to fail."] fn terminate (& mut self) ; # [doc = " Retrieve the current Actor execution state."] fn state (& self) -> ActorState ; }
};
}
