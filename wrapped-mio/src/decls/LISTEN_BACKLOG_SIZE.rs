macro_rules! LISTEN_BACKLOG_SIZE {
    () => {
        # [allow (dead_code)] # [cfg (not (any (target_os = "windows" , target_os = "redox" , target_os = "espidf" , target_os = "horizon" , target_os = "linux" , target_os = "freebsd" , target_os = "openbsd" , target_os = "wasi" , target_os = "hermit" , target_vendor = "apple" ,)))] pub (crate) const LISTEN_BACKLOG_SIZE : i32 = libc :: SOMAXCONN ;
    };
}

LISTEN_BACKLOG_SIZE!();