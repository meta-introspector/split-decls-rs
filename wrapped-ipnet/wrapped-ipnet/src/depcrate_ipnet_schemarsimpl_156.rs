// Generated macro for impl_156 (impl)
macro_rules! Depcrate_ipnet_schemarsimpl_156 {
() => {
// Module: crate::ipnet_schemars
// Provides: {"impl_156"}
// Dependencies: {}
impl JsonSchema for Ipv6Net { fn schema_name () -> String { "Ipv6Net" . to_string () } fn json_schema (_gen : & mut SchemaGenerator) -> Schema { Schema :: Object (SchemaObject { metadata : Some (Box :: new (Metadata { title : Some ("IPv6 network" . to_string ()) , description : Some ("An IPv6 address with prefix length" . to_string ()) , examples : vec ! [schemars :: _serde_json :: Value :: String ("::/0" . to_string ()) , schemars :: _serde_json :: Value :: String ("fd00::/32" . to_string ()) ,] , .. Default :: default () })) , instance_type : Some (SingleOrVec :: Single (Box :: new (InstanceType :: String))) , string : Some (Box :: new (StringValidation { max_length : Some (43) , min_length : None , pattern : Some (r#"^[0-9A-Fa-f:\.]+\/(?:[0-9]|[1-9][0-9]|1[0-1][0-9]|12[0-8])$"# . to_string ()) , .. Default :: default () })) , .. Default :: default () }) } }
};
}
