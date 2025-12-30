// Generated macro for impl_319 (impl)
macro_rules! Depcrate_arg_basic_implimpl_319 {
() => {
// Module: crate::arg::basic_impl
// Provides: {"impl_319"}
// Dependencies: {}
# [doc = " Represents a D-Bus string."] impl < 'a > Arg for & 'a str { const ARG_TYPE : ArgType = ArgType :: String ; fn signature () -> Signature < 'static > { unsafe { Signature :: from_slice_unchecked ("s\0") } } }
};
}
