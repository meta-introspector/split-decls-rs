// Generated macro for impl_332 (impl)
macro_rules! Depcrate_arg_basic_implimpl_332 {
() => {
// Module: crate::arg::basic_impl
// Provides: {"impl_332"}
// Dependencies: {}
impl Arg for OwnedFd { const ARG_TYPE : ArgType = ArgType :: UnixFd ; fn signature () -> Signature < 'static > { unsafe { Signature :: from_slice_unchecked ("h\0") } } }
};
}
