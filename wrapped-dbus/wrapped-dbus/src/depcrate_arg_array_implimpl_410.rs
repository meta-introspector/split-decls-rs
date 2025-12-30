// Generated macro for impl_410 (impl)
macro_rules! Depcrate_arg_array_implimpl_410 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_410"}
// Dependencies: {}
impl < K : DictKey + Append + Eq + Hash , V : Arg + Append , S : BuildHasher > Append for HashMap < K , V , S > { fn append_by_ref (& self , i : & mut IterAppend) { Dict :: new (self . iter ()) . append_by_ref (i) ; } }
};
}
