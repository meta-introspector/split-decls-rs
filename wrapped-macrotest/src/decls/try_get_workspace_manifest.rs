macro_rules! deps {
    () => {
        Result!();
        Error!();
        WorkspaceManifest!();
    };
}

macro_rules! try_get_workspace_manifest {
    () => {
        deps!();
        pub (crate) fn try_get_workspace_manifest (manifest_dir : & Path) -> Result < WorkspaceManifest , Error > { let cargo_toml_path = manifest_dir . join ("Cargo.toml") ; let manifest_str = fs :: read_to_string (cargo_toml_path) ? ; let mut manifest : WorkspaceManifest = toml :: de :: from_str (& manifest_str) ? ; fix_dependencies (& mut manifest . workspace . dependencies , manifest_dir) ; fix_patches (& mut manifest . patch , manifest_dir) ; fix_replacements (& mut manifest . replace , manifest_dir) ; Ok (manifest) }
    };
}

try_get_workspace_manifest!()