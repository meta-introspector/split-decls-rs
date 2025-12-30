// Generated macro for impl_145 (impl)
macro_rules! Depcrate_ipnet_serdeimpl_145 {
() => {
// Module: crate::ipnet_serde
// Provides: {"impl_145"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Ipv4Net { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > { if deserializer . is_human_readable () { struct IpAddrVisitor ; impl < 'de > Visitor < 'de > for IpAddrVisitor { type Value = Ipv4Net ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("IPv4 network address") } fn visit_str < E > (self , s : & str) -> Result < Self :: Value , E > where E : Error { s . parse () . map_err (Error :: custom) } } deserializer . deserialize_str (IpAddrVisitor) } else { let b = < [u8 ; 5] > :: deserialize (deserializer) ? ; Ipv4Net :: new (Ipv4Addr :: new (b [0] , b [1] , b [2] , b [3]) , b [4]) . map_err (serde :: de :: Error :: custom) } } }
};
}
