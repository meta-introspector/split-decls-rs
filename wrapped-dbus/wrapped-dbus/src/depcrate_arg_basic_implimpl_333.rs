// Generated macro for impl_333 (impl)
macro_rules! Depcrate_arg_basic_implimpl_333 {
() => {
// Module: crate::arg::basic_impl
// Provides: {"impl_333"}
// Dependencies: {}
impl Append for OwnedFd { # [cfg (unix)] fn append_by_ref (& self , i : & mut IterAppend) { arg_append_basic (& mut i . 0 , ArgType :: UnixFd , self . as_raw_fd ()) } # [cfg (windows)] fn append_by_ref (& self , _i : & mut IterAppend) { panic ! ("File descriptor passing not available on Windows") ; } }
};
}
