// Generated macro for impl_335 (impl)
macro_rules! Depcrate_arg_basic_implimpl_335 {
() => {
// Module: crate::arg::basic_impl
// Provides: {"impl_335"}
// Dependencies: {}
impl < 'a > Get < 'a > for OwnedFd { # [cfg (unix)] fn get (i : & mut Iter) -> Option < Self > { arg_get_basic (& mut i . 0 , ArgType :: UnixFd) . map (| fd | unsafe { OwnedFd :: from_raw_fd (fd) }) } # [cfg (windows)] fn get (_i : & mut Iter) -> Option < Self > { None } }
};
}
