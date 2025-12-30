// Generated macro for AnyFuture (struct)
macro_rules! Depcrate_streamAnyFuture {
() => {
// Module: crate::stream
// Provides: {"AnyFuture"}
// Dependencies: {}
# [doc = " Future for the [`StreamExt::any()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct AnyFuture < 'a , S : ? Sized , P > { stream : & 'a mut S , predicate : P , }
};
}
