// Generated macro for JoinFuture (enum)
macro_rules! Depcrate_join_allJoinFuture {
() => {
// Module: crate::join_all
// Provides: {"JoinFuture"}
// Dependencies: {}
enum JoinFuture < T > { Future (BoxFuture < 'static , T >) , Result (Option < T >) , }
};
}
