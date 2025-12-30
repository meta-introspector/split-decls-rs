// Generated macro for FindMapFuture (struct)
macro_rules! Depcrate_streamFindMapFuture {
() => {
// Module: crate::stream
// Provides: {"FindMapFuture"}
// Dependencies: {}
# [doc = " Future for the [`StreamExt::find_map()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct FindMapFuture < 'a , S : ? Sized , F > { stream : & 'a mut S , f : F , }
};
}
