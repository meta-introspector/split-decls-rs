macro_rules! home_dir_inner {
    () => {
        # [cfg (unix)] fn home_dir_inner () -> Option < PathBuf > { # [allow (deprecated)] std :: env :: home_dir () }
    };
}

home_dir_inner!()