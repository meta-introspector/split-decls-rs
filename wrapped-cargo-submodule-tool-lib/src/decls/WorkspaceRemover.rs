macro_rules! WorkspaceRemover {
    () => {
        pub trait WorkspaceRemover { fn remove_workspace_members (& self , cargo_toml_path : & Path , members_to_remove : & [String] ,) -> Result < () > ; }
    };
}

WorkspaceRemover!()