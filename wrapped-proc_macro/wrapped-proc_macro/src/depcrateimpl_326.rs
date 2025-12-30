// Generated macro for impl_326 (impl)
macro_rules! Depcrateimpl_326 {
() => {
// Module: crate
// Provides: {"impl_326"}
// Dependencies: {}
# [doc = " Collects a number of token trees into a single stream."] # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] impl FromIterator < TokenTree > for TokenStream { fn from_iter < I : IntoIterator < Item = TokenTree > > (trees : I) -> Self { let iter = trees . into_iter () ; let mut builder = ConcatTreesHelper :: new (iter . size_hint () . 0) ; iter . for_each (| tree | builder . push (tree)) ; builder . build () } }
};
}
