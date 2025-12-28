macro_rules! deps {
    () => {
        FileProcessingResult!();
    };
}

macro_rules! CollectedPreludeInfo {
    () => {
        deps!();
        # [derive (Serialize , Deserialize , Debug)] pub struct CollectedPreludeInfo { pub package_name : String , pub manifest_path : PathBuf , pub use_statements : std :: collections :: HashSet < String > , pub extern_crates : std :: collections :: HashSet < String > , pub feature_attributes : std :: collections :: HashSet < String > , pub crate_name : String , pub crate_root : PathBuf , pub prelude_content : String , pub modified_files : Vec < PathBuf > , pub crate_root_modified : bool , pub file_processing_results : Vec < FileProcessingResult > , }
    };
}

CollectedPreludeInfo!();