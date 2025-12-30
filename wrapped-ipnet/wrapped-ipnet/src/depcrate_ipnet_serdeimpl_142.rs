// Generated macro for impl_142 (impl)
macro_rules! Depcrate_ipnet_serdeimpl_142 {
() => {
// Module: crate::ipnet_serde
// Provides: {"impl_142"}
// Dependencies: {}
impl Serialize for IpNet { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer { if serializer . is_human_readable () { match * self { IpNet :: V4 (ref a) => a . serialize (serializer) , IpNet :: V6 (ref a) => a . serialize (serializer) , } } else { match * self { IpNet :: V4 (ref a) => serializer . serialize_newtype_variant ("IpNet" , 0 , "V4" , a) , IpNet :: V6 (ref a) => serializer . serialize_newtype_variant ("IpNet" , 1 , "V6" , a) , } } } }
};
}
