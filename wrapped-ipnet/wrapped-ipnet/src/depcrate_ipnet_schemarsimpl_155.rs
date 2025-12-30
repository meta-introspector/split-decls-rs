// Generated macro for impl_155 (impl)
macro_rules! Depcrate_ipnet_schemarsimpl_155 {
() => {
// Module: crate::ipnet_schemars
// Provides: {"impl_155"}
// Dependencies: {}
impl JsonSchema for Ipv4Net { fn schema_name () -> String { "Ipv4Net" . to_string () } fn json_schema (_gen : & mut SchemaGenerator) -> Schema { Schema :: Object (SchemaObject { metadata : Some (Box :: new (Metadata { title : Some ("IPv4 network" . to_string ()) , description : Some ("An IPv4 address with prefix length" . to_string ()) , examples : vec ! [schemars :: _serde_json :: Value :: String ("0.0.0.0/0" . to_string ()) , schemars :: _serde_json :: Value :: String ("192.168.0.0/24" . to_string ()) ,] , .. Default :: default () })) , instance_type : Some (SingleOrVec :: Single (Box :: new (InstanceType :: String))) , string : Some (Box :: new (StringValidation { max_length : Some (18) , min_length : None , pattern : Some (r#"^(?:(?:25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9][0-9]|[0-9])\.){3}(?:25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9][0-9]|[0-9])\/(?:3[0-2]|[1-2][0-9]|[0-9])$"# . to_string ()) , .. Default :: default () })) , .. Default :: default () }) } }
};
}
