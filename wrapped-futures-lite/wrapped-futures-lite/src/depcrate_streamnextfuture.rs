// Generated macro for NextFuture (struct)
macro_rules! Depcrate_streamNextFuture {
() => {
// Module: crate::stream
// Provides: {"NextFuture"}
// Dependencies: {}
# [doc = " Future for the [`StreamExt::next()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct NextFuture < 'a , S : ? Sized > { stream : & 'a mut S , }
};
}
