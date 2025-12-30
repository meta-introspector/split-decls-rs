// Generated macro for impl_414 (impl)
macro_rules! Depcrate_arg_array_implimpl_414 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_414"}
// Dependencies: {}
impl < K : DictKey + Append + Eq + Ord , V : Arg + Append > Append for BTreeMap < K , V > { fn append_by_ref (& self , i : & mut IterAppend) { Dict :: new (self . iter ()) . append_by_ref (i) ; } }
};
}
