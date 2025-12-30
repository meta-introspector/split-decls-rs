// Generated macro for NthFuture (struct)
macro_rules! Depcrate_streamNthFuture {
() => {
// Module: crate::stream
// Provides: {"NthFuture"}
// Dependencies: {}
# [doc = " Future for the [`StreamExt::nth()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct NthFuture < 'a , S : ? Sized > { stream : & 'a mut S , n : usize , }
};
}
