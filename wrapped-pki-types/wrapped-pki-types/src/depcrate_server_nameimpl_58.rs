// Generated macro for impl_58 (impl)
macro_rules! Depcrate_server_nameimpl_58 {
() => {
// Module: crate::server_name
// Provides: {"impl_58"}
// Dependencies: {}
impl ServerName < '_ > { # [doc = " Produce an owned `ServerName` from this (potentially borrowed) `ServerName`."] # [cfg (feature = "alloc")] pub fn to_owned (& self) -> ServerName < 'static > { match self { Self :: DnsName (d) => ServerName :: DnsName (d . to_owned ()) , Self :: IpAddress (i) => ServerName :: IpAddress (* i) , } } # [doc = " Return the string representation of this `ServerName`."] # [doc = ""] # [doc = " In the case of a `ServerName::DnsName` instance, this function returns a borrowed `str`."] # [doc = " For a `ServerName::IpAddress` instance it returns an allocated `String`."] # [cfg (feature = "std")] pub fn to_str (& self) -> Cow < '_ , str > { match self { Self :: DnsName (d) => d . as_ref () . into () , Self :: IpAddress (i) => std :: net :: IpAddr :: from (* i) . to_string () . into () , } } }
};
}
