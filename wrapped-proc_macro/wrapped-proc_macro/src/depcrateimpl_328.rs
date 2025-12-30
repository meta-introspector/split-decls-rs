// Generated macro for impl_328 (impl)
macro_rules! Depcrateimpl_328 {
() => {
// Module: crate
// Provides: {"impl_328"}
// Dependencies: {}
# [stable (feature = "token_stream_extend" , since = "1.30.0")] impl Extend < TokenTree > for TokenStream { fn extend < I : IntoIterator < Item = TokenTree > > (& mut self , trees : I) { let iter = trees . into_iter () ; let mut builder = ConcatTreesHelper :: new (iter . size_hint () . 0) ; iter . for_each (| tree | builder . push (tree)) ; builder . append_to (self) ; } }
};
}
