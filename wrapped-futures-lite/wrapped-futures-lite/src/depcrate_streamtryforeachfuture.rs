// Generated macro for TryForEachFuture (struct)
macro_rules! Depcrate_streamTryForEachFuture {
() => {
// Module: crate::stream
// Provides: {"TryForEachFuture"}
// Dependencies: {}
# [doc = " Future for the [`StreamExt::try_for_each()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryForEachFuture < 'a , S : ? Sized , F > { stream : & 'a mut S , f : F , }
};
}
