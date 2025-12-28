macro_rules! windows_sys {
    () => {
        # [cfg (any (windows , target_os = "cygwin"))] mod windows_sys ;
    };
}

windows_sys!();