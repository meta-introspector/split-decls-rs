macro_rules! macro_6 {
    () => {
        cfg_if ! { if # [cfg (any (target_os = "freebsd" , apple_targets ,))] { unsafe fn errno_location () -> * mut c_int { unsafe { libc :: __error () } } } else if # [cfg (any (target_os = "android" , netbsdlike , target_os = "cygwin"))] { unsafe fn errno_location () -> * mut c_int { unsafe { libc :: __errno () } } } else if # [cfg (any (target_os = "linux" , target_os = "redox" , target_os = "dragonfly" , target_os = "fuchsia" , target_os = "hurd" , target_os = "emscripten"))] { unsafe fn errno_location () -> * mut c_int { unsafe { libc :: __errno_location () } } } else if # [cfg (solarish)] { unsafe fn errno_location () -> * mut c_int { unsafe { libc :: ___errno () } } } else if # [cfg (any (target_os = "haiku" ,))] { unsafe fn errno_location () -> * mut c_int { unsafe { libc :: _errnop () } } } else if # [cfg (any (target_os = "aix"))] { unsafe fn errno_location () -> * mut c_int { unsafe { libc :: _Errno () } } } }
    };
}

macro_6!();