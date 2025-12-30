// Generated macro for impl_1496 (impl)
macro_rules! Depcrate_stream_try_stream_try_allimpl_1496 {
() => {
// Module: crate::stream::try_stream::try_all
// Provides: {"impl_1496"}
// Dependencies: {}
impl < St , Fut , F > TryAll < St , Fut , F > where St : TryStream , F : FnMut (St :: Ok) -> Fut , Fut : Future < Output = bool > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , done : false , future : None } } }
};
}
