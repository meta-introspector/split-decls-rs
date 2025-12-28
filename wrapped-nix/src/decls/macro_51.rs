macro_rules! macro_51 {
    () => {
        # [cfg (any (freebsdlike , all (target_os = "linux" , not (target_env = "ohos")) , target_os = "netbsd"))] feature ! { #! [feature = "mqueue"] pub mod mqueue ; }
    };
}

macro_51!();