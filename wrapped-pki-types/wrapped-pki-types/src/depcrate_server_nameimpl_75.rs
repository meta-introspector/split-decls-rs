// Generated macro for impl_75 (impl)
macro_rules! Depcrate_server_nameimpl_75 {
() => {
// Module: crate::server_name
// Provides: {"impl_75"}
// Dependencies: {}
impl AsRef < str > for DnsName < '_ > { fn as_ref (& self) -> & str { match self { Self (DnsNameInner :: Borrowed (s)) => s , # [cfg (feature = "alloc")] Self (DnsNameInner :: Owned (s)) => s . as_str () , } } }
};
}
