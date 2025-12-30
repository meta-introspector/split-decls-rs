// Generated macro for impl_78 (impl)
macro_rules! Depcrate_server_nameimpl_78 {
() => {
// Module: crate::server_name
// Provides: {"impl_78"}
// Dependencies: {}
impl Hash for DnsNameInner < '_ > { fn hash < H : Hasher > (& self , state : & mut H) { let s = match self { Self :: Borrowed (s) => s , # [cfg (feature = "alloc")] Self :: Owned (s) => s . as_str () , } ; s . chars () . for_each (| c | c . to_ascii_lowercase () . hash (state)) ; } }
};
}
