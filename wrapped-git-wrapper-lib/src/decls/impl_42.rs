macro_rules! deps {
    () => {
        RollupLock!();
        Result!();
        FileMetadata!();
        SubmoduleStat!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl RollupLock { pub fn new () -> Self { Default :: default () } # [cfg (feature = "serde_json_enabled")] pub fn load (root_dir : & Path) -> Result < Self > { let path = root_dir . join (Self :: FILE_NAME) ; if path . exists () { let content = std :: fs :: read_to_string (& path) . with_context (| | format ! ("Failed to read {}" , path . display ())) ? ; serde_json :: from_str (& content) . with_context (| | format ! ("Failed to parse {} as JSON" , path . display ())) } else { Ok (RollupLock :: default ()) } } # [cfg (not (feature = "serde_json_enabled"))] pub fn load (_root_dir : & Path) -> Result < Self > { Err (Box :: new (std :: io :: Error :: new (std :: io :: ErrorKind :: Other , "serde_json feature not enabled for RollupLock::load" ,)) . into ()) } # [cfg (feature = "serde_json_enabled")] pub fn save (& self , root_dir : & Path) -> Result < () > { let path = root_dir . join (Self :: FILE_NAME) ; let content = serde_json :: to_string_pretty (self) . with_context (| | { format ! ("Failed to serialize RollupLock to JSON for {}" , path . display ()) }) ? ; std :: fs :: write (& path , content) . with_context (| | format ! ("Failed to write to {}" , path . display ())) } # [cfg (not (feature = "serde_json_enabled"))] pub fn save (& self , _root_dir : & Path) -> Result < () > { Err (Box :: new (std :: io :: Error :: new (std :: io :: ErrorKind :: Other , "serde_json feature not enabled for RollupLock::save" ,)) . into ()) } pub fn get_metadata (& self , path : & Path) -> Option < & FileMetadata > { self . file_metadata_cache . get (path) } pub fn set_metadata (& mut self , path : PathBuf , metadata : FileMetadata) { self . file_metadata_cache . insert (path , metadata) ; } pub fn get_submodule_stat (& self , path : & Path) -> Option < & SubmoduleStat > { self . submodule_stat_cache . get (path) } pub fn set_submodule_stat (& mut self , path : PathBuf , stat : SubmoduleStat) { self . submodule_stat_cache . insert (path , stat) ; } }
    };
}

impl_42!()