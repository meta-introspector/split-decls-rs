macro_rules! uname {
    () => {
        # [cfg (any (target_os = "aix" , target_os = "dragonfly" , target_os = "freebsd" , target_os = "illumos" , target_os = "linux" , target_os = "macos" , target_os = "netbsd" , target_os = "openbsd" , target_os = "cygwin"))] mod uname ;
    };
}

uname!()