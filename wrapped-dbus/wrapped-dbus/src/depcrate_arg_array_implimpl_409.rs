// Generated macro for impl_409 (impl)
macro_rules! Depcrate_arg_array_implimpl_409 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_409"}
// Dependencies: {}
impl < K : DictKey , V : Arg , S : BuildHasher > Arg for HashMap < K , V , S > { const ARG_TYPE : ArgType = ArgType :: Array ; fn signature () -> Signature < 'static > { Signature :: from (format ! ("a{{{}{}}}" , K :: signature () , V :: signature ())) } }
};
}
