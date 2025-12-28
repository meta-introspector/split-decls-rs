macro_rules! home_dir_crt {
    () => {
        # [cfg (target_vendor = "uwp")] fn home_dir_crt () -> Option < PathBuf > { None }
    };
}

home_dir_crt!();