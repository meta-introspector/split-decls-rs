// Generated macro for load_workspace_from_metadata (function)
macro_rules! Depcrate_testsload_workspace_from_metadata {
() => {
// Module: crate::tests
// Provides: {"load_workspace_from_metadata"}
// Dependencies: {}
fn load_workspace_from_metadata (file : & str) -> ProjectWorkspace { let meta : Metadata = get_test_json_file (file) ; let manifest_path = ManifestPath :: try_from (AbsPathBuf :: try_from (meta . workspace_root . clone ()) . unwrap ()) . unwrap () ; let cargo_workspace = CargoWorkspace :: new (meta , manifest_path , Default :: default () , false) ; ProjectWorkspace { kind : ProjectWorkspaceKind :: Cargo { cargo : cargo_workspace , build_scripts : WorkspaceBuildScripts :: default () , rustc : Err (None) , error : None , } , cfg_overrides : Default :: default () , sysroot : Sysroot :: empty () , rustc_cfg : Vec :: new () , toolchain : None , target : Err ("target_data_layout not loaded" . into ()) , extra_includes : Vec :: new () , set_test : true , } }
};
}
