macro_rules! deps {
    () => {
        CurrentWalkDirIterator!();
        RepoDiscoverer!();
        RealCargoTomlProcessor!();
        PureRustRepoDiscoverer!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl RepoDiscoverer for PureRustRepoDiscoverer { fn discover_repos (& self , root_dir : & Path ,) -> std :: result :: Result < (Vec < RepoState > , HashSet < String >) , String > { let mut discovered_repos : HashSet < RepoState > = HashSet :: new () ; let mut all_vendored_crate_names : HashSet < String > = HashSet :: new () ; let walkdir_iterator = CurrentWalkDirIterator :: new (root_dir) ; use crate :: analysis :: cargo_toml_processor :: RealCargoTomlProcessor ; let cargo_toml_processor = RealCargoTomlProcessor ; for entry_result in walkdir_iterator . into_iter () { let path = entry_result . map_err (| e | format ! ("Error walking directory: {}" , e)) ? ; if path . ends_with ("target") || path . ends_with ("tests") || path . ends_with ("examples") || path . ends_with ("submodules/target") || path . ends_with ("submodules/tests") || path . ends_with ("submodules/examples") { continue ; } if path . file_name () . map_or (false , | name | name == "Cargo.toml") { cargo_toml_processor . process_cargo_toml (& path , & mut discovered_repos , & mut all_vendored_crate_names , & self . target_org , & self . target_branch ,) ? ; } } Ok ((discovered_repos . into_iter () . collect () , all_vendored_crate_names ,)) } }
    };
}

impl_144!();