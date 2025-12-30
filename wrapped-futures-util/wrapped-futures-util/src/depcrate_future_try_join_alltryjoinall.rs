// Generated macro for TryJoinAll (struct)
macro_rules! Depcrate_future_try_join_allTryJoinAll {
() => {
// Module: crate::future::try_join_all
// Provides: {"TryJoinAll"}
// Dependencies: {}
# [doc = " Future for the [`try_join_all`] function."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryJoinAll < F > where F : TryFuture , { kind : TryJoinAllKind < F > , }
};
}
