// Generated macro for tests (module)
macro_rules! Depcrate_hf_dataset_convertertests {
() => {
// Module: crate::hf_dataset_converter
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use std :: path :: Path ; # [tokio :: test] async fn test_hf_dataset_converter () { let base_path = "/home/mdupont/2025/08/07/solfunmeme-index" ; if ! Path :: new (base_path) . exists () { println ! ("Skipping test - dataset not found at {}" , base_path) ; return ; } let output_dir = "/tmp/test_hf_dataset" ; let converter = HuggingFaceDatasetConverter :: new (base_path , output_dir) . unwrap () ; let result = converter . create_dataset_config () ; assert ! (result . is_ok ()) ; let _ = fs :: remove_dir_all (output_dir) ; } }
};
}
