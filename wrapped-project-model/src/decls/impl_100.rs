macro_rules! deps {
    () => {
        ProjectWorkspaceKind!();
        ProjectWorkspace!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl fmt :: Debug for ProjectWorkspace { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let Self { kind , sysroot , rustc_cfg , toolchain , target : target_layout , cfg_overrides , extra_includes , set_test , } = self ; match kind { ProjectWorkspaceKind :: Cargo { cargo , error : _ , build_scripts , rustc } => f . debug_struct ("Cargo") . field ("root" , & cargo . workspace_root () . file_name ()) . field ("n_packages" , & cargo . packages () . len ()) . field ("n_sysroot_crates" , & sysroot . num_packages ()) . field ("n_rustc_compiler_crates" , & rustc . as_ref () . map (| a | a . as_ref ()) . map_or (0 , | (rc , _) | rc . packages () . len ()) ,) . field ("n_rustc_cfg" , & rustc_cfg . len ()) . field ("n_cfg_overrides" , & cfg_overrides . len ()) . field ("n_extra_includes" , & extra_includes . len ()) . field ("toolchain" , & toolchain) . field ("data_layout" , & target_layout) . field ("set_test" , set_test) . field ("build_scripts" , & build_scripts . error () . unwrap_or ("ok")) . finish () , ProjectWorkspaceKind :: Json (project) => { let mut debug_struct = f . debug_struct ("Json") ; debug_struct . field ("n_crates" , & project . n_crates ()) . field ("n_sysroot_crates" , & sysroot . num_packages ()) . field ("n_rustc_cfg" , & rustc_cfg . len ()) . field ("toolchain" , & toolchain) . field ("data_layout" , & target_layout) . field ("n_cfg_overrides" , & cfg_overrides . len ()) . field ("n_extra_includes" , & extra_includes . len ()) . field ("set_test" , set_test) ; debug_struct . finish () } ProjectWorkspaceKind :: DetachedFile { file , cargo : cargo_script } => f . debug_struct ("DetachedFiles") . field ("file" , & file) . field ("cargo_script" , & cargo_script . is_some ()) . field ("n_sysroot_crates" , & sysroot . num_packages ()) . field ("n_rustc_cfg" , & rustc_cfg . len ()) . field ("toolchain" , & toolchain) . field ("data_layout" , & target_layout) . field ("n_cfg_overrides" , & cfg_overrides . len ()) . field ("n_extra_includes" , & extra_includes . len ()) . field ("set_test" , set_test) . finish () , } } }
    };
}

impl_100!()