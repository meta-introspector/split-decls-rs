// Generated macro for impl_745 (impl)
macro_rules! Depcrate_stream_stream_fuseimpl_745 {
() => {
// Module: crate::stream::stream::fuse
// Provides: {"impl_745"}
// Dependencies: {}
impl < St > Fuse < St > { pub (crate) fn new (stream : St) -> Self { Self { stream , done : false } } # [doc = " Returns whether the underlying stream has finished or not."] # [doc = ""] # [doc = " If this method returns `true`, then all future calls to poll are"] # [doc = " guaranteed to return `None`. If this returns `false`, then the"] # [doc = " underlying stream is still in use."] pub fn is_done (& self) -> bool { self . done } delegate_access_inner ! (stream , St , ()) ; }
};
}
