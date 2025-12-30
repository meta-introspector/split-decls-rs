// Generated macro for impl_345 (impl)
macro_rules! Depcrate_arg_basic_implimpl_345 {
() => {
// Module: crate::arg::basic_impl
// Provides: {"impl_345"}
// Dependencies: {}
impl < 'a > Get < 'a > for File { # [cfg (unix)] fn get (i : & mut Iter) -> Option < Self > { arg_get_basic (& mut i . 0 , ArgType :: UnixFd) . map (| fd | unsafe { File :: from_raw_fd (fd) }) } # [cfg (windows)] fn get (_i : & mut Iter) -> Option < Self > { None } }
};
}
