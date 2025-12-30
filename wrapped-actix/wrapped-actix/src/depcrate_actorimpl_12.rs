// Generated macro for impl_12 (impl)
macro_rules! Depcrate_actorimpl_12 {
() => {
// Module: crate::actor
// Provides: {"impl_12"}
// Dependencies: {}
impl ActorState { # [doc = " Indicates whether the actor is alive."] pub fn alive (self) -> bool { self == ActorState :: Started || self == ActorState :: Running } # [doc = " Indicates whether the actor is stopped or stopping."] pub fn stopping (self) -> bool { self == ActorState :: Stopping || self == ActorState :: Stopped } }
};
}
