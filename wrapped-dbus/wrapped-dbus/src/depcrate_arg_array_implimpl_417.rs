// Generated macro for impl_417 (impl)
macro_rules! Depcrate_arg_array_implimpl_417 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_417"}
// Dependencies: {}
impl < T : Arg > Arg for Vec < T > { const ARG_TYPE : ArgType = ArgType :: Array ; fn signature () -> Signature < 'static > { Signature :: from (format ! ("a{}" , T :: signature ())) } }
};
}
