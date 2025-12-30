// Generated macro for impl_339 (impl)
macro_rules! Depcrate_arg_basic_implimpl_339 {
() => {
// Module: crate::arg::basic_impl
// Provides: {"impl_339"}
// Dependencies: {}
# [cfg (all (unix , feature = "io-lifetimes"))] impl < 'a > Get < 'a > for io_lifetimes :: OwnedFd { fn get (i : & mut Iter) -> Option < Self > { arg_get_basic (& mut i . 0 , ArgType :: UnixFd) . map (| fd | unsafe { io_lifetimes :: OwnedFd :: from_raw_fd (fd) }) } }
};
}
