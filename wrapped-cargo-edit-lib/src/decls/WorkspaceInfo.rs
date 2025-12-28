macro_rules! WorkspaceInfo {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq)] pub struct WorkspaceInfo { pub member_crates : Vec < String > , pub submodule_base_path_rel : std :: path :: PathBuf , }
    };
}

WorkspaceInfo!()