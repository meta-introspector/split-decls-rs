// Generated macro for DatasetInfo (struct)
macro_rules! Depcrate_hf_dataset_converterDatasetInfo {
() => {
// Module: crate::hf_dataset_converter
// Provides: {"DatasetInfo"}
// Dependencies: {}
# [doc = " Dataset info structure for Hugging Face"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DatasetInfo { pub description : String , pub citation : String , pub homepage : String , pub license : String , pub features : HashMap < String , FeatureInfo > , pub splits : HashMap < String , SplitInfo > , pub download_size : u64 , pub dataset_size : u64 , pub config_name : String , pub dataset_name : String , pub version : String , }
};
}
