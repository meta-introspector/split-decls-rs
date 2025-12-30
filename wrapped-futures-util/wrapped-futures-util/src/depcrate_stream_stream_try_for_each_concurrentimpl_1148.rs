// Generated macro for impl_1148 (impl)
macro_rules! Depcrate_stream_stream_try_for_each_concurrentimpl_1148 {
() => {
// Module: crate::stream::stream::try_for_each_concurrent
// Provides: {"impl_1148"}
// Dependencies: {}
impl < St , Fut , F , E > TryForEachConcurrent < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = Result < () , E > > , { pub (super) fn new (stream : St , limit : Option < usize > , f : F) -> Self { Self { stream : Some (stream) , limit : limit . and_then (NonZeroUsize :: new) , f , futures : FuturesUnordered :: new () , } } }
};
}
