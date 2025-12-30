// Generated macro for impl_691 (impl)
macro_rules! Depcrate_stream_stream_anyimpl_691 {
() => {
// Module: crate::stream::stream::any
// Provides: {"impl_691"}
// Dependencies: {}
impl < St , Fut , F > Any < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = bool > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , done : false , future : None } } }
};
}
