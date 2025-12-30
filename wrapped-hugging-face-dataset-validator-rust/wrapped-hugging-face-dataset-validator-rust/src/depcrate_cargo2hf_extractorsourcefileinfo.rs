// Generated macro for SourceFileInfo (struct)
macro_rules! Depcrate_cargo2hf_extractorSourceFileInfo {
() => {
// Module: crate::cargo2hf_extractor
// Provides: {"SourceFileInfo"}
// Dependencies: {}
# [doc = " Source code file analysis"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct SourceFileInfo { # [doc = " Relative path from project root"] pub path : String , # [doc = " File type (lib, bin, test, example, bench)"] pub file_type : String , # [doc = " Lines of code in this file"] pub lines_of_code : u32 , # [doc = " Number of functions defined"] pub function_count : u32 , # [doc = " Number of structs defined"] pub struct_count : u32 , # [doc = " Number of enums defined"] pub enum_count : u32 , # [doc = " Number of traits defined"] pub trait_count : u32 , # [doc = " Number of impl blocks"] pub impl_count : u32 , # [doc = " Number of public items"] pub public_items : u32 , # [doc = " Estimated complexity score for this file"] pub complexity_score : f32 , # [doc = " Documentation coverage for this file"] pub doc_coverage : f32 , }
};
}
