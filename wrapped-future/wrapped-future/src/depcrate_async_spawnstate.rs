// Generated macro for State (struct)
macro_rules! Depcrate_async_spawnState {
() => {
// Module: crate::async_spawn
// Provides: {"State"}
// Dependencies: {}
struct State < T : Async > { result : Option < Result < T :: Output > > , completed : Option < T :: CompletedHandler > , completed_assigned : bool , }
};
}
