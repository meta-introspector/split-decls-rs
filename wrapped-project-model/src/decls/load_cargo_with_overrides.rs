macro_rules! deps {
    () => {
        ProjectWorkspace!();
        CfgOverrides!();
    };
}

macro_rules! load_cargo_with_overrides {
    () => {
        deps!();
        fn load_cargo_with_overrides (file : & str , cfg_overrides : CfgOverrides ,) -> (CrateGraphBuilder , ProcMacroPaths) { let project_workspace = ProjectWorkspace { cfg_overrides , .. load_workspace_from_metadata (file) } ; to_crate_graph (project_workspace , & mut Default :: default ()) }
    };
}

load_cargo_with_overrides!();