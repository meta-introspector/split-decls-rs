// Generated macro for impl_1338 (impl)
macro_rules! Depcrate_stream_try_stream_try_collectimpl_1338 {
() => {
// Module: crate::stream::try_stream::try_collect
// Provides: {"impl_1338"}
// Dependencies: {}
impl < St : TryStream , C : Default > TryCollect < St , C > { pub (super) fn new (s : St) -> Self { Self { stream : s , items : Default :: default () } } }
};
}
