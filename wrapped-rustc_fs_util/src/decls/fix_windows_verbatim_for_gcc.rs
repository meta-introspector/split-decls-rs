macro_rules! fix_windows_verbatim_for_gcc {
    () => {
        # [cfg (not (windows))] pub fn fix_windows_verbatim_for_gcc (p : & Path) -> PathBuf { p . to_path_buf () }
    };
}

fix_windows_verbatim_for_gcc!()