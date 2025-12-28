macro_rules! deps {
    () => {
        SourceRootConfig!();
    };
}

macro_rules! ProjectFolders {
    () => {
        deps!();
        # [derive (Default)] pub struct ProjectFolders { pub load : Vec < vfs :: loader :: Entry > , pub watch : Vec < usize > , pub source_root_config : SourceRootConfig , }
    };
}

ProjectFolders!();