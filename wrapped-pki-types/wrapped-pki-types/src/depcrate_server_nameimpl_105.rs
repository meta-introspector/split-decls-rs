// Generated macro for impl_105 (impl)
macro_rules! Depcrate_server_nameimpl_105 {
() => {
// Module: crate::server_name
// Provides: {"impl_105"}
// Dependencies: {}
impl core :: fmt :: Display for AddrParseError { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . write_str (match self . 0 { AddrKind :: Ipv4 => "invalid IPv4 address syntax" , AddrKind :: Ipv6 => "invalid IPv6 address syntax" , }) } }
};
}
