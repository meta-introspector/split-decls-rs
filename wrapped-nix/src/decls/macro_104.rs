macro_rules! macro_104 {
    () => {
        # [cfg (any (linux_android , target_os = "freebsd"))] feature ! { #! [feature = "fs"] pub mod memfd ; }
    };
}

macro_104!();