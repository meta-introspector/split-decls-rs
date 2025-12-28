macro_rules! macro_111 {
    () => {
        # [cfg (any (target_os = "linux" , netbsdlike))] feature ! { #! [feature = "reboot"] pub mod reboot ; }
    };
}

macro_111!()