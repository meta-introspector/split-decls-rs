macro_rules! linux_and_more {
    () => {
        # [cfg (not (any (target_os = "windows" , target_os = "macos")))] mod linux_and_more ;
    };
}

linux_and_more!();