// Generated macro for impl_405 (impl)
macro_rules! Depcrate_arg_array_implimpl_405 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_405"}
// Dependencies: {}
impl < 'a , K : DictKey , V : Arg , I > Arg for Dict < 'a , K , V , I > { const ARG_TYPE : ArgType = ArgType :: Array ; fn signature () -> Signature < 'static > { Signature :: from (format ! ("a{}" , Self :: entry_sig ())) } }
};
}
