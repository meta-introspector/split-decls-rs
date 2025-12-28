macro_rules! deps {
    () => {
        SysrootPublicDeps!();
    };
}

macro_rules! extend_crate_graph_with_sysroot {
    () => {
        deps!();
        fn extend_crate_graph_with_sysroot (crate_graph : & mut CrateGraphBuilder , mut sysroot_crate_graph : CrateGraphBuilder , mut sysroot_proc_macros : ProcMacroPaths ,) -> (SysrootPublicDeps , Option < CrateBuilderId >) { let mut pub_deps = vec ! [] ; let mut libproc_macro = None ; for cid in sysroot_crate_graph . iter () { if let CrateOrigin :: Lang (lang_crate) = sysroot_crate_graph [cid] . basic . origin { match lang_crate { LangCrateOrigin :: Test | LangCrateOrigin :: Alloc | LangCrateOrigin :: Core | LangCrateOrigin :: Std => pub_deps . push ((CrateName :: normalize_dashes (& lang_crate . to_string ()) , cid , ! matches ! (lang_crate , LangCrateOrigin :: Test | LangCrateOrigin :: Alloc) ,)) , LangCrateOrigin :: ProcMacro => libproc_macro = Some (cid) , LangCrateOrigin :: Other => () , } } } let mut marker_set = vec ! [] ; for & (_ , cid , _) in pub_deps . iter () { marker_set . extend (sysroot_crate_graph . transitive_deps (cid)) ; } if let Some (cid) = libproc_macro { marker_set . extend (sysroot_crate_graph . transitive_deps (cid)) ; } marker_set . sort () ; marker_set . dedup () ; let removed_mapping = sysroot_crate_graph . remove_crates_except (& marker_set) ; sysroot_proc_macros = sysroot_proc_macros . into_iter () . filter_map (| (k , v) | Some ((removed_mapping [k . into_raw () . into_u32 () as usize] ? , v))) . collect () ; let mapping = crate_graph . extend (sysroot_crate_graph , & mut sysroot_proc_macros) ; pub_deps . iter_mut () . for_each (| (_ , cid , _) | { * cid = mapping [& removed_mapping [cid . into_raw () . into_u32 () as usize] . unwrap ()] }) ; if let Some (libproc_macro) = & mut libproc_macro { * libproc_macro = mapping [& removed_mapping [libproc_macro . into_raw () . into_u32 () as usize] . unwrap ()] ; } (SysrootPublicDeps { deps : pub_deps } , libproc_macro) }
    };
}

extend_crate_graph_with_sysroot!();