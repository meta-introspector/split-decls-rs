// Generated macro for impl_157 (impl)
macro_rules! Depcrate_ipnet_schemarsimpl_157 {
() => {
// Module: crate::ipnet_schemars
// Provides: {"impl_157"}
// Dependencies: {}
impl JsonSchema for IpNet { fn schema_name () -> String { "IpNet" . to_string () } fn json_schema (gen : & mut SchemaGenerator) -> Schema { Schema :: Object (SchemaObject { metadata : Some (Box :: new (Metadata { title : Some ("IP network" . to_string ()) , description : Some ("An IPv4 or IPv6 address with prefix length" . to_string ()) , examples : vec ! [schemars :: _serde_json :: Value :: String ("192.168.0.0/24" . to_string ()) , schemars :: _serde_json :: Value :: String ("fd00::/32" . to_string ()) ,] , .. Default :: default () })) , subschemas : Some (Box :: new (SubschemaValidation { one_of : Some (vec ! [Ipv4Net :: json_schema (gen) , Ipv6Net :: json_schema (gen)]) , .. Default :: default () })) , .. Default :: default () }) } }
};
}
