macro_rules! imp {
    () => {
        # [cfg (not (any (target_os = "aix" , target_os = "android" , target_os = "dragonfly" , target_os = "emscripten" , target_os = "freebsd" , target_os = "illumos" , target_os = "ios" , target_os = "linux" , target_os = "macos" , target_os = "netbsd" , target_os = "openbsd" , target_os = "cygwin" , target_os = "redox" , target_os = "windows")))] # [path = "unknown/mod.rs"] mod imp ;
    };
}

imp!();