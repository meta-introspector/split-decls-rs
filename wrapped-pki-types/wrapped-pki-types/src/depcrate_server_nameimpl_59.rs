// Generated macro for impl_59 (impl)
macro_rules! Depcrate_server_nameimpl_59 {
() => {
// Module: crate::server_name
// Provides: {"impl_59"}
// Dependencies: {}
impl fmt :: Debug for ServerName < '_ > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { Self :: DnsName (d) => f . debug_tuple ("DnsName") . field (& d . as_ref ()) . finish () , Self :: IpAddress (i) => f . debug_tuple ("IpAddress") . field (i) . finish () , } } }
};
}
