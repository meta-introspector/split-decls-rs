// Generated macro for impl_79 (impl)
macro_rules! Depcrate_server_nameimpl_79 {
() => {
// Module: crate::server_name
// Provides: {"impl_79"}
// Dependencies: {}
impl fmt :: Debug for DnsNameInner < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Borrowed (s) => f . write_fmt (format_args ! ("{s:?}")) , # [cfg (feature = "alloc")] Self :: Owned (s) => f . write_fmt (format_args ! ("{s:?}")) , } } }
};
}
