macro_rules! SplitExpandedBinInputs {
    () => {
        # [derive (Debug)] pub struct SplitExpandedBinInputs < 'a > { pub files_to_process : Vec < PathBuf > , pub project_root : PathBuf , pub rustc_version : String , pub rustc_host : String , pub verbose : u8 , pub output_global_toml : Option < PathBuf > , pub output_symbol_map : Option < PathBuf > , pub warnings : & 'a mut Vec < String > , pub canonical_output_root : & 'a Path , }
    };
}

SplitExpandedBinInputs!();