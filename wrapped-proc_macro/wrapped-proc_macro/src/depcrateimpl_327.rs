// Generated macro for impl_327 (impl)
macro_rules! Depcrateimpl_327 {
() => {
// Module: crate
// Provides: {"impl_327"}
// Dependencies: {}
# [doc = " A \"flattening\" operation on token streams, collects token trees"] # [doc = " from multiple token streams into a single stream."] # [stable (feature = "proc_macro_lib" , since = "1.15.0")] impl FromIterator < TokenStream > for TokenStream { fn from_iter < I : IntoIterator < Item = TokenStream > > (streams : I) -> Self { let iter = streams . into_iter () ; let mut builder = ConcatStreamsHelper :: new (iter . size_hint () . 0) ; iter . for_each (| stream | builder . push (stream)) ; builder . build () } }
};
}
