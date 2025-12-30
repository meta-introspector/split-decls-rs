// Generated macro for workspace_members (function)
macro_rules! Depcrate_core_metadataworkspace_members {
() => {
// Module: crate::core::metadata
// Provides: {"workspace_members"}
// Dependencies: {}
# [doc = " Invokes `cargo metadata` to get package metadata of each workspace member."] # [doc = ""] # [doc = " This is used to resolve specific crate paths in `fn should_run` to compile"] # [doc = " particular crate (e.g., `x build sysroot` to build library/sysroot)."] fn workspace_members (build : & Build) -> Vec < Package > { let collect_metadata = | manifest_path | { let mut cargo = command (& build . initial_cargo) ; cargo . env ("RUSTC_BOOTSTRAP" , "1") . arg ("metadata") . arg ("--format-version") . arg ("1") . arg ("--no-deps") . arg ("--manifest-path") . arg (build . src . join (manifest_path)) ; let metadata_output = cargo . run_in_dry_run () . run_capture_stdout (build) . stdout () ; let Output { packages , .. } = t ! (serde_json :: from_str (& metadata_output)) ; packages } ; let mut packages = vec ! [] ; packages . extend (collect_metadata ("Cargo.toml")) ; packages . extend (collect_metadata ("library/Cargo.toml")) ; packages }
};
}
