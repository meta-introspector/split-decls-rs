macro_rules! macro_55 {
    () => {
        # [cfg (any (freebsdlike , all (target_os = "linux" , not (any (target_env = "uclibc" , target_env = "ohos"))) , apple_targets , target_os = "netbsd"))] feature ! { #! [feature = "aio"] pub mod aio ; }
    };
}

macro_55!();