macro_rules! deps {
    () => {
        WorkspaceManifest!();
    };
}

macro_rules! get_workspace_manifest {
    () => {
        deps!();
        pub (crate) fn get_workspace_manifest (manifest_dir : & Path) -> WorkspaceManifest { try_get_workspace_manifest (manifest_dir) . unwrap_or_default () }
    };
}

get_workspace_manifest!()