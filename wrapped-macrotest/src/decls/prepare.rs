macro_rules! deps {
    () => {
        Error!();
        ExpandedTest!();
        Project!();
        Result!();
    };
}

macro_rules! prepare {
    () => {
        deps!();
        fn prepare (tests : & [ExpandedTest]) -> Result < Project > { let metadata = cargo :: metadata () ? ; let target_dir = metadata . target_directory ; let workspace = metadata . workspace_root ; let crate_name = env :: var ("CARGO_PKG_NAME") . map_err (| _ | Error :: PkgName) ? ; let source_dir = env :: var_os ("CARGO_MANIFEST_DIR") . map (PathBuf :: from) . ok_or (Error :: ManifestDir) ? ; let features = features :: find () ; let overwrite = match env :: var_os ("MACROTEST") { Some (ref v) if v == "overwrite" => true , Some (v) => return Err (Error :: UnrecognizedEnv (v)) , None => false , } ; let random_string : String = iter :: repeat_with (fastrand :: alphanumeric) . take (42) . collect () ; let dir = path ! (target_dir / "tests" / crate_name / random_string) ; if dir . exists () { fs :: remove_dir_all (& dir) ? ; } let inner_target_dir = path ! (target_dir / "tests" / "macrotest") ; let mut project = Project { dir , source_dir , inner_target_dir , name : format ! ("{}-tests" , crate_name) , features , workspace , overwrite , } ; let manifest = make_manifest (crate_name , & project , tests) ? ; let manifest_toml = toml :: ser :: to_string (& manifest) ? ; let config = make_config () ; let config_toml = toml :: ser :: to_string (& config) ? ; if let Some (enabled_features) = & mut project . features { enabled_features . retain (| feature | manifest . features . contains_key (feature)) ; } fs :: create_dir_all (path ! (project . dir / ".cargo")) ? ; fs :: write (path ! (project . dir / ".cargo" / "config.toml") , config_toml) ? ; fs :: write (path ! (project . dir / "Cargo.toml") , manifest_toml) ? ; fs :: write (path ! (project . dir / "main.rs") , b"fn main() {}\n") ? ; let source_lockfile = path ! (project . workspace / "Cargo.lock") ; match fs :: copy (source_lockfile , path ! (project . dir / "Cargo.lock")) { Err (e) if e . kind () == std :: io :: ErrorKind :: NotFound => Ok (0) , otherwise => otherwise , } ? ; fs :: create_dir_all (& project . inner_target_dir) ? ; cargo :: build_dependencies (& project) ? ; Ok (project) }
    };
}

prepare!()