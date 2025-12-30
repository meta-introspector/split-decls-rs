// Generated macro for State (struct)
macro_rules! Depcrate_rt_condvarState {
() => {
// Module: crate::rt::condvar
// Provides: {"State"}
// Dependencies: {}
# [derive (Debug)] pub (super) struct State { # [doc = " Tracks access to the mutex"] last_access : Option < Access > , # [doc = " Threads waiting on the condvar"] waiters : VecDeque < thread :: Id > , }
};
}
