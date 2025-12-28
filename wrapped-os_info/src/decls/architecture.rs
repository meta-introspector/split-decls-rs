macro_rules! architecture {
    () => {
        # [cfg (any (target_os = "linux" , target_os = "macos" , target_os = "netbsd" , target_os = "openbsd" , target_os = "cygwin"))] mod architecture ;
    };
}

architecture!();