// Generated macro for impl_395 (impl)
macro_rules! Depcrate_arg_array_implimpl_395 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_395"}
// Dependencies: {}
# [doc = " Represents a D-Bus array."] impl < 'a , T : Arg > Arg for & 'a [T] { const ARG_TYPE : ArgType = ArgType :: Array ; fn signature () -> Signature < 'static > { Signature :: from (format ! ("a{}" , T :: signature ())) } }
};
}
