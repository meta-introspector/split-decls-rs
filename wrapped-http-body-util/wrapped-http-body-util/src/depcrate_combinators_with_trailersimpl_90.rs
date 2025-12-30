// Generated macro for impl_90 (impl)
macro_rules! Depcrate_combinators_with_trailersimpl_90 {
() => {
// Module: crate::combinators::with_trailers
// Provides: {"impl_90"}
// Dependencies: {}
impl < T , F > WithTrailers < T , F > { pub (crate) fn new (body : T , trailers : F) -> Self { Self { state : State :: PollBody { body , trailers : Some (trailers) , } , } } }
};
}
