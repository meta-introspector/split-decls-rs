// Generated macro for impl_411 (impl)
macro_rules! Depcrate_arg_array_implimpl_411 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_411"}
// Dependencies: {}
impl < 'a , K : DictKey + Get < 'a > + Eq + Hash , V : Arg + Get < 'a > , S : BuildHasher + Default > Get < 'a > for HashMap < K , V , S > { fn get (i : & mut Iter < 'a >) -> Option < Self > { Dict :: get (i) . map (| d | d . collect ()) } }
};
}
