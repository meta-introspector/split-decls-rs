// Generated macro for FeatureInfo (struct)
macro_rules! Depcrate_hf_dataset_converterFeatureInfo {
() => {
// Module: crate::hf_dataset_converter
// Provides: {"FeatureInfo"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct FeatureInfo { pub dtype : String , pub description : String , # [serde (skip_serializing_if = "Option::is_none")] pub class_label : Option < Vec < String > > , }
};
}
