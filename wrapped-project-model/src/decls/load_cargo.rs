macro_rules! load_cargo {
    () => {
        fn load_cargo (file : & str) -> (CrateGraphBuilder , ProcMacroPaths) { let project_workspace = load_workspace_from_metadata (file) ; to_crate_graph (project_workspace , & mut Default :: default ()) }
    };
}

load_cargo!()