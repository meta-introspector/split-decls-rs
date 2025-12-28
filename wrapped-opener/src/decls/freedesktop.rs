macro_rules! freedesktop {
    () => {
        # [cfg (all (feature = "reveal" , target_os = "linux"))] mod freedesktop ;
    };
}

freedesktop!();