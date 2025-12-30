// Generated macro for State (struct)
macro_rules! Depcrate_rt_rwlockState {
() => {
// Module: crate::rt::rwlock
// Provides: {"State"}
// Dependencies: {}
# [derive (Debug)] pub (super) struct State { # [doc = " A single `thread::Id` when Write locked."] # [doc = " A set of `thread::Id` when Read locked."] lock : Option < Locked > , # [doc = " Tracks write access to the rwlock."] last_access : Option < Access > , # [doc = " Causality transfers between threads"] synchronize : Synchronize , }
};
}
