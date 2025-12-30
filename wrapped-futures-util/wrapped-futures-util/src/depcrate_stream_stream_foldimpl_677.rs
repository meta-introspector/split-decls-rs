// Generated macro for impl_677 (impl)
macro_rules! Depcrate_stream_stream_foldimpl_677 {
() => {
// Module: crate::stream::stream::fold
// Provides: {"impl_677"}
// Dependencies: {}
impl < St , Fut , T , F > Fold < St , Fut , T , F > where St : Stream , F : FnMut (T , St :: Item) -> Fut , Fut : Future < Output = T > , { pub (super) fn new (stream : St , f : F , t : T) -> Self { Self { stream , f , accum : Some (t) , future : None } } }
};
}
