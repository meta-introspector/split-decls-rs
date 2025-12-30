// Generated macro for Compat (struct)
macro_rules! Depcrate_compat_compat03as01Compat {
() => {
// Module: crate::compat::compat03as01
// Provides: {"Compat"}
// Dependencies: {}
# [allow (clippy :: too_long_first_doc_paragraph)] # [doc = " Converts a futures 0.3 [`TryFuture`](futures_core::future::TryFuture) or"] # [doc = " [`TryStream`](futures_core::stream::TryStream) into a futures 0.1"] # [doc = " [`Future`](futures_01::future::Future) or"] # [doc = " [`Stream`](futures_01::stream::Stream)."] # [derive (Debug , Clone , Copy)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Compat < T > { pub (crate) inner : T , }
};
}
