// Generated macro for impl_422 (impl)
macro_rules! Depcrate_arg_array_implimpl_422 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_422"}
// Dependencies: {}
impl < 'a , T : Arg , I > Arg for Array < 'a , T , I > { const ARG_TYPE : ArgType = ArgType :: Array ; fn signature () -> Signature < 'static > { Signature :: from (format ! ("a{}" , T :: signature ())) } }
};
}
