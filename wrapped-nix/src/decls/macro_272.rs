macro_rules! macro_272 {
    () => {
        # [cfg (any (target_os = "freebsd" , target_os = "haiku" , target_os = "linux" , target_os = "netbsd" , apple_targets))] feature ! { #! [feature = "process"] pub mod spawn ; }
    };
}

macro_272!()