// Generated macro for impl_343 (impl)
macro_rules! Depcrate_arg_basic_implimpl_343 {
() => {
// Module: crate::arg::basic_impl
// Provides: {"impl_343"}
// Dependencies: {}
impl Append for File { # [cfg (unix)] fn append_by_ref (& self , i : & mut IterAppend) { arg_append_basic (& mut i . 0 , ArgType :: UnixFd , self . as_raw_fd ()) } # [cfg (windows)] fn append_by_ref (& self , _i : & mut IterAppend) { panic ! ("File descriptor passing not available on Windows") ; } }
};
}
