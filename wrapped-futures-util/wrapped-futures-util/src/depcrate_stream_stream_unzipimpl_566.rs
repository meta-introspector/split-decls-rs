// Generated macro for impl_566 (impl)
macro_rules! Depcrate_stream_stream_unzipimpl_566 {
() => {
// Module: crate::stream::stream::unzip
// Provides: {"impl_566"}
// Dependencies: {}
impl < St : Stream , FromA : Default , FromB : Default > Unzip < St , FromA , FromB > { fn finish (self : Pin < & mut Self >) -> (FromA , FromB) { let this = self . project () ; (mem :: take (this . left) , mem :: take (this . right)) } pub (super) fn new (stream : St) -> Self { Self { stream , left : Default :: default () , right : Default :: default () } } }
};
}
