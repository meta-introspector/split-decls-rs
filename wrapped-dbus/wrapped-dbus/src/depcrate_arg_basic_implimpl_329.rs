// Generated macro for impl_329 (impl)
macro_rules! Depcrate_arg_basic_implimpl_329 {
() => {
// Module: crate::arg::basic_impl
// Provides: {"impl_329"}
// Dependencies: {}
# [doc = " Represents a D-Bus string."] impl < 'a > Arg for & 'a CStr { const ARG_TYPE : ArgType = ArgType :: String ; fn signature () -> Signature < 'static > { unsafe { Signature :: from_slice_unchecked ("s\0") } } }
};
}
