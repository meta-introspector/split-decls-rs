// Generated macro for impl_413 (impl)
macro_rules! Depcrate_arg_array_implimpl_413 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_413"}
// Dependencies: {}
impl < K : DictKey , V : Arg > Arg for BTreeMap < K , V > { const ARG_TYPE : ArgType = ArgType :: Array ; fn signature () -> Signature < 'static > { Signature :: from (format ! ("a{{{}{}}}" , K :: signature () , V :: signature ())) } }
};
}
