macro_rules! deps {
    () => {
        RealWorkspaceRemover!();
        WorkspaceRemover!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        # [cfg (not (feature = "cargo-toml-editor-lib"))] impl WorkspaceRemover for RealWorkspaceRemover { fn remove_workspace_members (& self , _cargo_toml_path : & Path , _members_to_remove : & [String] ,) -> Result < () > { Err (anyhow ! ("`WorkspaceRemover` requires the `cargo-toml-editor-lib` feature to be enabled.")) } }
    };
}

impl_53!();