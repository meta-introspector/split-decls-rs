// Generated macro for impl_282 (impl)
macro_rules! Depcrate_stream_collectimpl_282 {
() => {
// Module: crate::stream::collect
// Provides: {"impl_282"}
// Dependencies: {}
impl < S : Stream > Collect < S > { fn finish (& mut self) -> Vec < S :: Item > { mem :: replace (& mut self . items , Vec :: new ()) } }
};
}
