macro_rules! macro_137 {
    () => {
        # [cfg (any (linux_android , freebsdlike , apple_targets , target_os = "openbsd" , target_os = "cygwin"))] feature ! { #! [feature = "fs"] pub mod statfs ; }
    };
}

macro_137!()