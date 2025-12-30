// Generated macro for impl_705 (impl)
macro_rules! Depcrate_stream_stream_allimpl_705 {
() => {
// Module: crate::stream::stream::all
// Provides: {"impl_705"}
// Dependencies: {}
impl < St , Fut , F > All < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = bool > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , done : false , future : None } } }
};
}
