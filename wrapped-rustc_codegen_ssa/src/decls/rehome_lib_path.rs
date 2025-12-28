macro_rules! rehome_lib_path {
    () => {
        fn rehome_lib_path (sess : & Session , path : & Path) -> PathBuf { if let Some (dir) = path . parent () { let file_name = path . file_name () . expect ("library path has no file name component") ; rehome_sysroot_lib_dir (sess , dir) . join (file_name) } else { fix_windows_verbatim_for_gcc (path) } }
    };
}

rehome_lib_path!()