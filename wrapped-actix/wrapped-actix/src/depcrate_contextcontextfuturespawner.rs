// Generated macro for ContextFutureSpawner (trait)
macro_rules! Depcrate_contextContextFutureSpawner {
() => {
// Module: crate::context
// Provides: {"ContextFutureSpawner"}
// Dependencies: {}
# [doc = " Helper trait which can spawn a future into the actor's context."] pub trait ContextFutureSpawner < A > where A : Actor , A :: Context : AsyncContext < A > , { # [doc = " Spawns the future into the given context."] fn spawn (self , ctx : & mut A :: Context) ; # [doc = " Spawns the future into the given context, waiting for it to"] # [doc = " resolve."] # [doc = ""] # [doc = " This stops processing any incoming events until this future"] # [doc = " resolves."] fn wait (self , ctx : & mut A :: Context) ; }
};
}
