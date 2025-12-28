macro_rules! deps {
    () => {
        Result!();
        RepoStateCollector!();
        SubmoduleInfo!();
        RealRepoStateCollector!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl RepoStateCollector for RealRepoStateCollector { fn collect_repo_state (& self , project_root : & Path) -> Result < RepoState > { let mut repo_state = RepoState :: default () ; for (url , path) in self . git_executor . list_submodules (project_root) ? { let name = path . file_name () . and_then (| os_str | os_str . to_str ()) . unwrap_or ("unknown") . to_string () ; repo_state . submodules . push (SubmoduleInfo { name , path , url , branch : None , commit_id : "unknown" . to_string () , }) ; } for entry in WalkDir :: new (project_root) . into_iter () . filter_map (| e | e . ok ()) . filter (| e | e . file_type () . is_file () && e . file_name () == "Cargo.toml") { let manifest_path = entry . path () . to_path_buf () ; let metadata = self . cargo_metadata_provider . provide_metadata (& manifest_path) ? ; let mut packages_in_workspace = Vec :: new () ; for pkg in metadata . packages { let mut deps_info = Vec :: new () ; for dep in pkg . dependencies { deps_info . push (DependencyInfo { name : dep . name , source : dep . source . as_ref () . map (| s | s . to_string ()) . unwrap_or_else (| | "path" . to_string ()) , req : dep . req . to_string () , }) ; } packages_in_workspace . push (PackageInfo { name : pkg . name . to_string () , version : pkg . version . to_string () , manifest_path : PathBuf :: from (pkg . manifest_path) , dependencies : deps_info , }) ; } let workspace_root_str = metadata . workspace_root . as_str () ; if ! workspace_root_str . is_empty () { if Path :: new (workspace_root_str) == manifest_path . parent () . unwrap () { repo_state . cargo_workspaces . push (CargoWorkspaceInfo { manifest_path , packages : packages_in_workspace , }) ; } } else { repo_state . cargo_workspaces . push (CargoWorkspaceInfo { manifest_path , packages : packages_in_workspace , }) ; } } for entry in WalkDir :: new (project_root) . into_iter () . filter_map (| e | e . ok ()) . filter (| e | { e . file_type () . is_file () && (e . file_name () == "flake.nix" || e . file_name () == "Cargo.nix") }) { let flake_path = entry . path () . to_path_buf () ; repo_state . nix_flakes . push (NixFlakeInfo { flake_path , inputs : BTreeMap :: new () , outputs : Vec :: new () , }) ; } Ok (repo_state) } }
    };
}

impl_66!()