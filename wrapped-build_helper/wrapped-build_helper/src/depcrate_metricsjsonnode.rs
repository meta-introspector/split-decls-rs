// Generated macro for JsonNode (enum)
macro_rules! Depcrate_metricsJsonNode {
() => {
// Module: crate::metrics
// Provides: {"JsonNode"}
// Dependencies: {}
# [derive (Serialize , Deserialize)] # [serde (tag = "kind" , rename_all = "snake_case")] pub enum JsonNode { RustbuildStep { # [serde (rename = "type")] type_ : String , debug_repr : String , # [serde (deserialize_with = "null_as_f64_nan")] duration_excluding_children_sec : f64 , system_stats : JsonStepSystemStats , children : Vec < JsonNode > , } , TestSuite (TestSuite) , }
};
}
