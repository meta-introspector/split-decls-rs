macro_rules! deps {
    () => {
        Result!();
        Manifest!();
        Error!();
    };
}

macro_rules! try_get_manifest {
    () => {
        deps!();
        fn try_get_manifest (manifest_dir : & Path) -> Result < Manifest , Error > { let cargo_toml_path = manifest_dir . join ("Cargo.toml") ; let manifest_str = fs :: read_to_string (cargo_toml_path) ? ; let mut manifest : Manifest = toml :: de :: from_str (& manifest_str) ? ; fix_dependencies (& mut manifest . dependencies , manifest_dir) ; fix_dependencies (& mut manifest . dev_dependencies , manifest_dir) ; Ok (manifest) }
    };
}

try_get_manifest!()