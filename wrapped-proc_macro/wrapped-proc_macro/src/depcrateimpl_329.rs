// Generated macro for impl_329 (impl)
macro_rules! Depcrateimpl_329 {
() => {
// Module: crate
// Provides: {"impl_329"}
// Dependencies: {}
# [stable (feature = "token_stream_extend" , since = "1.30.0")] impl Extend < TokenStream > for TokenStream { fn extend < I : IntoIterator < Item = TokenStream > > (& mut self , streams : I) { let iter = streams . into_iter () ; let mut builder = ConcatStreamsHelper :: new (iter . size_hint () . 0) ; iter . for_each (| stream | builder . push (stream)) ; builder . append_to (self) ; } }
};
}
