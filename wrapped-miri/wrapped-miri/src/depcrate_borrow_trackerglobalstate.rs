// Generated macro for GlobalState (type)
macro_rules! Depcrate_borrow_trackerGlobalState {
() => {
// Module: crate::borrow_tracker
// Provides: {"GlobalState"}
// Dependencies: {}
# [doc = " We need interior mutable access to the global state."] pub type GlobalState = RefCell < GlobalStateInner > ;
};
}
