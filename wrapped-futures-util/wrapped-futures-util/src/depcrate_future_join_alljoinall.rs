// Generated macro for JoinAll (struct)
macro_rules! Depcrate_future_join_allJoinAll {
() => {
// Module: crate::future::join_all
// Provides: {"JoinAll"}
// Dependencies: {}
# [must_use = "futures do nothing unless you `.await` or poll them"] # [doc = " Future for the [`join_all`] function."] pub struct JoinAll < F > where F : Future , { kind : JoinAllKind < F > , }
};
}
