// Generated macro for DatasetConfig (struct)
macro_rules! Depcrate_hf_dataset_converterDatasetConfig {
() => {
// Module: crate::hf_dataset_converter
// Provides: {"DatasetConfig"}
// Dependencies: {}
# [doc = " Standard Hugging Face dataset configuration"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DatasetConfig { pub dataset_name : String , pub description : String , pub version : String , pub license : String , pub homepage : String , pub repository : String , pub tags : Vec < String > , pub task_categories : Vec < String > , pub language : Vec < String > , pub size_categories : String , }
};
}
