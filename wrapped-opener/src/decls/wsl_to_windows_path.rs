macro_rules! wsl_to_windows_path {
    () => {
        # [cfg (not (target_os = "linux"))] fn wsl_to_windows_path (_path : & OsStr) -> Option < OsString > { unreachable ! () }
    };
}

wsl_to_windows_path!();