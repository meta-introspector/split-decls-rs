macro_rules! is_wsl {
    () => {
        # [cfg (not (target_os = "linux"))] fn is_wsl () -> bool { false }
    };
}

is_wsl!()