// Generated macro for impl_336 (impl)
macro_rules! Depcrate_arg_basic_implimpl_336 {
() => {
// Module: crate::arg::basic_impl
// Provides: {"impl_336"}
// Dependencies: {}
# [cfg (all (unix , feature = "io-lifetimes"))] impl Arg for io_lifetimes :: OwnedFd { const ARG_TYPE : ArgType = ArgType :: UnixFd ; fn signature () -> Signature < 'static > { unsafe { Signature :: from_slice_unchecked ("h\0") } } }
};
}
