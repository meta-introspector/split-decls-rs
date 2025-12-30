// Generated macro for load_workspace_at (function)
macro_rules! Depcrateload_workspace_at {
() => {
// Module: crate
// Provides: {"load_workspace_at"}
// Dependencies: {}
pub fn load_workspace_at (root : & Path , cargo_config : & CargoConfig , load_config : & LoadCargoConfig , progress : & (dyn Fn (String) + Sync) ,) -> anyhow :: Result < (RootDatabase , vfs :: Vfs , Option < ProcMacroClient >) > { let root = AbsPathBuf :: assert_utf8 (std :: env :: current_dir () ? . join (root)) ; let root = ProjectManifest :: discover_single (& root) ? ; let manifest_path = root . manifest_path () . clone () ; let mut workspace = ProjectWorkspace :: load (root , cargo_config , progress) ? ; if load_config . load_out_dirs_from_check { let build_scripts = workspace . run_build_scripts (cargo_config , progress) ? ; if let Some (error) = build_scripts . error () { tracing :: debug ! ("Errors occurred while running build scripts for {}: {}" , manifest_path , error) ; } workspace . set_build_scripts (build_scripts) } load_workspace (workspace , & cargo_config . extra_env , load_config) }
};
}
