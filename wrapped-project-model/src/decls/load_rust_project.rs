macro_rules! deps {
    () => {
        ProjectWorkspace!();
        ProjectWorkspaceKind!();
        Sysroot!();
    };
}

macro_rules! load_rust_project {
    () => {
        deps!();
        fn load_rust_project (file : & str) -> (CrateGraphBuilder , ProcMacroPaths) { let data = get_test_json_file (file) ; let project = rooted_project_json (data) ; let sysroot = Sysroot :: empty () ; let project_workspace = ProjectWorkspace { kind : ProjectWorkspaceKind :: Json (project) , sysroot , rustc_cfg : Vec :: new () , toolchain : None , target : Err ("test has no target data" . into ()) , cfg_overrides : Default :: default () , extra_includes : Vec :: new () , set_test : true , } ; to_crate_graph (project_workspace , & mut Default :: default ()) }
    };
}

load_rust_project!()