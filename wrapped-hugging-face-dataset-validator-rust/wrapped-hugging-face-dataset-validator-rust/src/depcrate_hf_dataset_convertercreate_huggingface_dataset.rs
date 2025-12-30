// Generated macro for create_huggingface_dataset (function)
macro_rules! Depcrate_hf_dataset_convertercreate_huggingface_dataset {
() => {
// Module: crate::hf_dataset_converter
// Provides: {"create_huggingface_dataset"}
// Dependencies: {}
# [doc = " CLI function to create Hugging Face dataset"] pub async fn create_huggingface_dataset (base_path : & str , output_dir : & str ,) -> Result < () , ValidationError > { let converter = HuggingFaceDatasetConverter :: new (base_path , output_dir) ? ; converter . create_huggingface_dataset () . await }
};
}
