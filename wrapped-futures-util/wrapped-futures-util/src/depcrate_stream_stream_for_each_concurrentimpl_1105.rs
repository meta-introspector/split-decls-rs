// Generated macro for impl_1105 (impl)
macro_rules! Depcrate_stream_stream_for_each_concurrentimpl_1105 {
() => {
// Module: crate::stream::stream::for_each_concurrent
// Provides: {"impl_1105"}
// Dependencies: {}
impl < St , Fut , F > ForEachConcurrent < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = () > , { pub (super) fn new (stream : St , limit : Option < usize > , f : F) -> Self { Self { stream : Some (stream) , limit : limit . and_then (NonZeroUsize :: new) , f , futures : FuturesUnordered :: new () , } } }
};
}
