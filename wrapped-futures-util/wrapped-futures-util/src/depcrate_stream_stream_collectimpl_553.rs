// Generated macro for impl_553 (impl)
macro_rules! Depcrate_stream_stream_collectimpl_553 {
() => {
// Module: crate::stream::stream::collect
// Provides: {"impl_553"}
// Dependencies: {}
impl < St : Stream , C : Default > Collect < St , C > { fn finish (self : Pin < & mut Self >) -> C { mem :: take (self . project () . collection) } pub (super) fn new (stream : St) -> Self { Self { stream , collection : Default :: default () } } }
};
}
