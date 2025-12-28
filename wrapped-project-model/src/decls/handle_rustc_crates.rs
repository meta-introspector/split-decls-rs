macro_rules! deps {
    () => {
        CfgOverrides!();
        WorkspaceBuildScripts!();
        TargetKind!();
        CargoWorkspace!();
        FileLoader!();
        SysrootPublicDeps!();
        Package!();
    };
}

macro_rules! handle_rustc_crates {
    () => {
        deps!();
        fn handle_rustc_crates (crate_graph : & mut CrateGraphBuilder , proc_macros : & mut ProcMacroPaths , pkg_to_lib_crate : & mut FxHashMap < Package , CrateBuilderId > , load : FileLoader < '_ > , rustc_workspace : & CargoWorkspace , cargo : & CargoWorkspace , public_deps : & SysrootPublicDeps , libproc_macro : Option < CrateBuilderId > , pkg_crates : & FxHashMap < Package , Vec < (CrateBuilderId , TargetKind) > > , cfg_options : & CfgOptions , override_cfg : & CfgOverrides , build_scripts : & WorkspaceBuildScripts , crate_ws_data : Arc < CrateWorkspaceData > , cargo_path : & Utf8Path ,) { let mut rustc_pkg_crates = FxHashMap :: default () ; let root_pkg = rustc_workspace . packages () . find (| & package | rustc_workspace [package] . name == "rustc_driver") ; let workspace_proc_macro_cwd = Arc :: new (cargo . workspace_root () . to_path_buf ()) ; if let Some (root_pkg) = root_pkg { let mut queue = VecDeque :: new () ; queue . push_back (root_pkg) ; while let Some (pkg) = queue . pop_front () { if rustc_pkg_crates . contains_key (& pkg) { continue ; } let pkg_data = & rustc_workspace [pkg] ; for dep in & pkg_data . dependencies { queue . push_back (dep . pkg) ; } let mut cfg_options = cfg_options . clone () ; override_cfg . apply (& mut cfg_options , & pkg_data . name) ; for & tgt in pkg_data . targets . iter () { let kind @ TargetKind :: Lib { is_proc_macro } = rustc_workspace [tgt] . kind else { continue ; } ; let pkg_crates = & mut rustc_pkg_crates . entry (pkg) . or_insert_with (Vec :: new) ; if let Some (file_id) = load (& rustc_workspace [tgt] . root) { let crate_id = add_target_crate_root (crate_graph , proc_macros , rustc_workspace , pkg_data , build_scripts . get_output (pkg) . zip (Some (build_scripts . error () . is_some ())) , cfg_options . clone () , file_id , & rustc_workspace [tgt] . name , kind , CrateOrigin :: Rustc { name : Symbol :: intern (& pkg_data . name) } , crate_ws_data . clone () , if pkg_data . is_member { workspace_proc_macro_cwd . clone () } else { Arc :: new (pkg_data . manifest . parent () . to_path_buf ()) } , cargo_path ,) ; pkg_to_lib_crate . insert (pkg , crate_id) ; public_deps . add_to_crate_graph (crate_graph , crate_id) ; if let Some (proc_macro) = libproc_macro { add_proc_macro_dep (crate_graph , crate_id , proc_macro , is_proc_macro) ; } pkg_crates . push (crate_id) ; } } } } for pkg in rustc_pkg_crates . keys () . copied () { for dep in rustc_workspace [pkg] . dependencies . iter () { let name = CrateName :: new (& dep . name) . unwrap () ; if let Some (& to) = pkg_to_lib_crate . get (& dep . pkg) { for & from in rustc_pkg_crates . get (& pkg) . into_iter () . flatten () { add_dep (crate_graph , from , name . clone () , to) ; } } } } for dep in rustc_workspace . packages () { let name = CrateName :: normalize_dashes (& rustc_workspace [dep] . name) ; if let Some (& to) = pkg_to_lib_crate . get (& dep) { for pkg in cargo . packages () { let package = & cargo [pkg] ; if ! package . metadata . rustc_private { continue ; } for (from , _) in pkg_crates . get (& pkg) . into_iter () . flatten () { if ! crate_graph [* from] . basic . dependencies . iter () . any (| d | d . name == name) { add_dep (crate_graph , * from , name . clone () , to) ; } } } } } }
    };
}

handle_rustc_crates!()