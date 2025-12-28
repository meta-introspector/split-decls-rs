macro_rules! deps {
    () => {
        RealWorkspaceRemover!();
        WorkspaceRemover!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        # [cfg (feature = "cargo-toml-editor-lib")] impl WorkspaceRemover for RealWorkspaceRemover { fn remove_workspace_members (& self , cargo_toml_path : & Path , members_to_remove : & [String] ,) -> Result < () > { anyhow :: bail ! ("`RealWorkspaceRemover` is not implemented without `cargo-toml-editor-lib` feature.") ; } }
    };
}

impl_51!()