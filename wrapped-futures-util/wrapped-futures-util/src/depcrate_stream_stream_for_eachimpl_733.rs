// Generated macro for impl_733 (impl)
macro_rules! Depcrate_stream_stream_for_eachimpl_733 {
() => {
// Module: crate::stream::stream::for_each
// Provides: {"impl_733"}
// Dependencies: {}
impl < St , Fut , F > ForEach < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = () > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , future : None } } }
};
}
