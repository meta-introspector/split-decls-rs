macro_rules! deps {
    () => {
        SubmoduleStat!();
        Serialize!();
        FileMetadata!();
        Deserialize!();
    };
}

macro_rules! RollupLock {
    () => {
        deps!();
        # [cfg_attr (feature = "serde_enabled" , derive (Debug , Serialize , Deserialize , Default))] # [cfg_attr (not (feature = "serde_enabled") , derive (Debug , Default))] pub struct RollupLock { pub file_metadata_cache : HashMap < PathBuf , FileMetadata > , pub submodule_stat_cache : HashMap < PathBuf , SubmoduleStat > , pub git_tree_cache : HashMap < PathBuf , String > , pub last_snapshot_time : Option < SystemTime > , pub project_root_hash : Option < String > , pub crate_hashes : HashMap < PathBuf , String > , pub submodule_hashes : HashMap < PathBuf , String > , pub cargo_toml_hashes : HashMap < PathBuf , String > , pub rust_file_hashes : HashMap < PathBuf , String > , }
    };
}

RollupLock!();