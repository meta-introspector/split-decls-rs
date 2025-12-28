macro_rules! tz_data {
    () => {
        # [cfg (all (any (target_os = "android" , target_env = "ohos" , test) , feature = "clock"))] mod tz_data ;
    };
}

tz_data!();