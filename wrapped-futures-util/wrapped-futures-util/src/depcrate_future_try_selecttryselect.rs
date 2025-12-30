// Generated macro for TrySelect (struct)
macro_rules! Depcrate_future_try_selectTrySelect {
() => {
// Module: crate::future::try_select
// Provides: {"TrySelect"}
// Dependencies: {}
# [doc = " Future for the [`try_select()`] function."] # [must_use = "futures do nothing unless you `.await` or poll them"] # [derive (Debug)] pub struct TrySelect < A , B > { inner : Option < (A , B) > , }
};
}
