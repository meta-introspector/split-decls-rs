// Generated macro for impl_147 (impl)
macro_rules! Depcrate_ipnet_serdeimpl_147 {
() => {
// Module: crate::ipnet_serde
// Provides: {"impl_147"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Ipv6Net { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > { if deserializer . is_human_readable () { struct IpAddrVisitor ; impl < 'de > Visitor < 'de > for IpAddrVisitor { type Value = Ipv6Net ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("IPv6 network address") } fn visit_str < E > (self , s : & str) -> Result < Self :: Value , E > where E : Error { s . parse () . map_err (Error :: custom) } } deserializer . deserialize_str (IpAddrVisitor) } else { let b = < [u8 ; 17] > :: deserialize (deserializer) ? ; Ipv6Net :: new (Ipv6Addr :: new (((b [0] as u16) << 8) | b [1] as u16 , ((b [2] as u16) << 8) | b [3] as u16 , ((b [4] as u16) << 8) | b [5] as u16 , ((b [6] as u16) << 8) | b [7] as u16 , ((b [8] as u16) << 8) | b [9] as u16 , ((b [10] as u16) << 8) | b [11] as u16 , ((b [12] as u16) << 8) | b [13] as u16 , ((b [14] as u16) << 8) | b [15] as u16) , b [16]) . map_err (Error :: custom) } } }
};
}
