macro_rules! deps {
    () => {
        Serialize!();
        Deserialize!();
    };
}

macro_rules! FileMetadata {
    () => {
        deps!();
        # [cfg_attr (feature = "serde_enabled" , derive (Debug , PartialEq , Clone , Serialize , Deserialize))] # [cfg_attr (not (feature = "serde_enabled") , derive (Debug , PartialEq , Clone))] pub struct FileMetadata { pub modified : SystemTime , pub len : u64 , pub hash : String , pub git_object_hash : Option < String > , pub is_git_tracked : bool , }
    };
}

FileMetadata!();