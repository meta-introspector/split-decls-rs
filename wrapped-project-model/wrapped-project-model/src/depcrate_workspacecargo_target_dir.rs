// Generated macro for cargo_target_dir (function)
macro_rules! Depcrate_workspacecargo_target_dir {
() => {
// Module: crate::workspace
// Provides: {"cargo_target_dir"}
// Dependencies: {}
fn cargo_target_dir (manifest : & ManifestPath , extra_env : & FxHashMap < String , Option < String > > , sysroot : & Sysroot ,) -> Option < Utf8PathBuf > { let cargo = sysroot . tool (Tool :: Cargo , manifest . parent () , extra_env) ; let mut meta = cargo_metadata :: MetadataCommand :: new () ; meta . cargo_path (cargo . get_program ()) ; meta . manifest_path (manifest) ; meta . no_deps () ; let mut other_options = vec ! [] ; if manifest . is_rust_manifest () { meta . env ("RUSTC_BOOTSTRAP" , "1") ; other_options . push ("-Zscript" . to_owned ()) ; } meta . other_options (other_options) ; meta . exec () . map (| m | m . target_directory) . ok () }
};
}
