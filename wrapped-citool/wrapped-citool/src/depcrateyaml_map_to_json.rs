// Generated macro for yaml_map_to_json (function)
macro_rules! Depcrateyaml_map_to_json {
() => {
// Module: crate
// Provides: {"yaml_map_to_json"}
// Dependencies: {}
fn yaml_map_to_json (map : & BTreeMap < String , Value >) -> BTreeMap < String , serde_json :: Value > { map . into_iter () . map (| (key , value) | { (key . clone () , serde_json :: to_value (& value) . expect ("Cannot convert map value from YAML to JSON") ,) }) . collect () }
};
}
