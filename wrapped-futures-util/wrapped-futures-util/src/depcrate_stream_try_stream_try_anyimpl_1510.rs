// Generated macro for impl_1510 (impl)
macro_rules! Depcrate_stream_try_stream_try_anyimpl_1510 {
() => {
// Module: crate::stream::try_stream::try_any
// Provides: {"impl_1510"}
// Dependencies: {}
impl < St , Fut , F > TryAny < St , Fut , F > where St : TryStream , F : FnMut (St :: Ok) -> Fut , Fut : Future < Output = bool > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , done : false , future : None } } }
};
}
