// Generated macro for LimitState (struct)
macro_rules! Depcrate_executorLimitState {
() => {
// Module: crate::executor
// Provides: {"LimitState"}
// Dependencies: {}
struct LimitState { count : Cell < usize > , deferred : RefCell < Vec < Box < ExecuteCallback > > > , }
};
}
