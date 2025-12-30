// Generated macro for impl_77 (impl)
macro_rules! Depcrate_server_nameimpl_77 {
() => {
// Module: crate::server_name
// Provides: {"impl_77"}
// Dependencies: {}
impl PartialEq < Self > for DnsNameInner < '_ > { fn eq (& self , other : & Self) -> bool { match (self , other) { (Self :: Borrowed (s) , Self :: Borrowed (o)) => s . eq_ignore_ascii_case (o) , # [cfg (feature = "alloc")] (Self :: Borrowed (s) , Self :: Owned (o)) => s . eq_ignore_ascii_case (o . as_str ()) , # [cfg (feature = "alloc")] (Self :: Owned (s) , Self :: Borrowed (o)) => s . eq_ignore_ascii_case (o) , # [cfg (feature = "alloc")] (Self :: Owned (s) , Self :: Owned (o)) => s . eq_ignore_ascii_case (o . as_str ()) , } } }
};
}
