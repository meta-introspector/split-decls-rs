// Generated macro for State (struct)
macro_rules! Depcrate_rt_mutexState {
() => {
// Module: crate::rt::mutex
// Provides: {"State"}
// Dependencies: {}
# [derive (Debug)] pub (super) struct State { # [doc = " If the mutex should establish sequential consistency."] seq_cst : bool , # [doc = " `Some` when the mutex is in the locked state. The `thread::Id`"] # [doc = " references the thread that currently holds the mutex."] lock : Option < thread :: Id > , # [doc = " Tracks access to the mutex"] last_access : Option < Access > , # [doc = " Causality transfers between threads"] synchronize : Synchronize , }
};
}
