macro_rules! WorkspaceGenerator {
    () => {
        pub trait WorkspaceGenerator { fn generate_workspace_dependencies (& self , root_dir : & Path , dry_run : bool , executor : Arc < dyn Execv + Send + Sync > ,) -> anyhow :: Result < () > ; }
    };
}

WorkspaceGenerator!()