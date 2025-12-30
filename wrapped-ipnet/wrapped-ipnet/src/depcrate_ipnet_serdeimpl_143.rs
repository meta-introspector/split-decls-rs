// Generated macro for impl_143 (impl)
macro_rules! Depcrate_ipnet_serdeimpl_143 {
() => {
// Module: crate::ipnet_serde
// Provides: {"impl_143"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for IpNet { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > { if deserializer . is_human_readable () { struct IpNetVisitor ; impl < 'de > Visitor < 'de > for IpNetVisitor { type Value = IpNet ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("IPv4 or IPv6 network address") } fn visit_str < E > (self , s : & str) -> Result < Self :: Value , E > where E : Error { s . parse () . map_err (Error :: custom) } } deserializer . deserialize_str (IpNetVisitor) } else { struct EnumVisitor ; # [derive (Serialize , Deserialize)] enum IpNetKind { V4 , V6 , } impl < 'de > Visitor < 'de > for EnumVisitor { type Value = IpNet ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("IPv4 or IPv6 network address") } fn visit_enum < A > (self , data : A) -> Result < Self :: Value , A :: Error > where A : EnumAccess < 'de > { match data . variant () ? { (IpNetKind :: V4 , v) => v . newtype_variant () . map (IpNet :: V4) , (IpNetKind :: V6 , v) => v . newtype_variant () . map (IpNet :: V6) , } } } deserializer . deserialize_enum ("IpNet" , & ["V4" , "V6"] , EnumVisitor) } } }
};
}
