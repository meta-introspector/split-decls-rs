// Generated macro for impl_415 (impl)
macro_rules! Depcrate_arg_array_implimpl_415 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_415"}
// Dependencies: {}
impl < 'a , K : DictKey + Get < 'a > + Eq + Ord , V : Arg + Get < 'a > > Get < 'a > for BTreeMap < K , V > { fn get (i : & mut Iter < 'a >) -> Option < Self > { Dict :: get (i) . map (| d | d . collect ()) } }
};
}
