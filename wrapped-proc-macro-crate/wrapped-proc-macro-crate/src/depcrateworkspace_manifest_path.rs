// Generated macro for workspace_manifest_path (function)
macro_rules! Depcrateworkspace_manifest_path {
() => {
// Module: crate
// Provides: {"workspace_manifest_path"}
// Dependencies: {}
fn workspace_manifest_path (cargo_toml_manifest : & Path) -> Result < Option < PathBuf > , Error > { let Ok (cargo) = env :: var ("CARGO") else { return Ok (None) ; } ; let stdout = Command :: new (cargo) . arg ("locate-project") . args (& ["--workspace" , "--message-format=plain"]) . arg (format ! ("--manifest-path={}" , cargo_toml_manifest . display ())) . output () . map_err (| _ | Error :: FailedGettingWorkspaceManifestPath) ? . stdout ; String :: from_utf8 (stdout) . map_err (| _ | Error :: FailedGettingWorkspaceManifestPath) . map (| s | { let path = s . trim () ; if path . is_empty () { None } else { Some (path . into ()) } }) }
};
}
