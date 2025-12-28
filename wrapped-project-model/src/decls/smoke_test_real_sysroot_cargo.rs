macro_rules! deps {
    () => {
        ManifestPath!();
        RustSourceWorkspaceConfig!();
        CargoWorkspace!();
        ProjectWorkspace!();
        WorkspaceBuildScripts!();
        ProjectWorkspaceKind!();
        Sysroot!();
        RustLibSrcWorkspace!();
    };
}

macro_rules! smoke_test_real_sysroot_cargo {
    () => {
        deps!();
        # [test] fn smoke_test_real_sysroot_cargo () { let file_map = & mut FxHashMap :: < AbsPathBuf , FileId > :: default () ; let meta : Metadata = get_test_json_file ("hello-world-metadata.json") ; let manifest_path = ManifestPath :: try_from (AbsPathBuf :: try_from (meta . workspace_root . clone ()) . unwrap ()) . unwrap () ; let cargo_workspace = CargoWorkspace :: new (meta , manifest_path , Default :: default () , false) ; let mut sysroot = Sysroot :: discover (AbsPath :: assert (Utf8Path :: new (env ! ("CARGO_MANIFEST_DIR"))) , & Default :: default () ,) ; let cwd = AbsPathBuf :: assert_utf8 (temp_dir () . join ("smoke_test_real_sysroot_cargo")) ; std :: fs :: create_dir_all (& cwd) . unwrap () ; let loaded_sysroot = sysroot . load_workspace (& RustSourceWorkspaceConfig :: default_cargo () , false , & | _ | ()) ; if let Some (loaded_sysroot) = loaded_sysroot { sysroot . set_workspace (loaded_sysroot) ; } assert ! (matches ! (sysroot . workspace () , RustLibSrcWorkspace :: Workspace { .. }) , "got {}" , sysroot . workspace ()) ; let project_workspace = ProjectWorkspace { kind : ProjectWorkspaceKind :: Cargo { cargo : cargo_workspace , build_scripts : WorkspaceBuildScripts :: default () , rustc : Err (None) , error : None , } , sysroot , rustc_cfg : Vec :: new () , cfg_overrides : Default :: default () , toolchain : None , target : Err ("target_data_layout not loaded" . into ()) , extra_includes : Vec :: new () , set_test : true , } ; project_workspace . to_crate_graph (& mut { | path | { let len = file_map . len () ; Some (* file_map . entry (path . to_path_buf ()) . or_insert (FileId :: from_raw (len as u32))) } } , & Default :: default () ,) ; }
    };
}

smoke_test_real_sysroot_cargo!();