// Generated macro for PoolState (struct)
macro_rules! Depcrate_poolPoolState {
() => {
// Module: crate::pool
// Provides: {"PoolState"}
// Dependencies: {}
struct PoolState { workers : RefCell < Vec < Worker > > , callback : Closure < dyn FnMut (Event) > , }
};
}
