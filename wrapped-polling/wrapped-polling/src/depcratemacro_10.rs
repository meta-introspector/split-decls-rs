// Generated macro for macro_10 (macro)
macro_rules! Depcratemacro_10 {
() => {
// Module: crate
// Provides: {"macro_10"}
// Dependencies: {}
cfg_if ! { if # [cfg (polling_test_poll_backend)] { mod poll ; use poll as sys ; } else if # [cfg (any (target_os = "linux" , target_os = "android" , target_os = "redox"))] { mod epoll ; use epoll as sys ; } else if # [cfg (any (target_os = "illumos" , target_os = "solaris" ,))] { mod port ; use port as sys ; } else if # [cfg (any (target_vendor = "apple" , target_os = "freebsd" , target_os = "netbsd" , target_os = "openbsd" , target_os = "dragonfly" ,))] { mod kqueue ; use kqueue as sys ; } else if # [cfg (any (target_os = "vxworks" , target_os = "hermit" , target_os = "fuchsia" , target_os = "horizon" , unix ,))] { mod poll ; use poll as sys ; } else if # [cfg (target_os = "windows")] { mod iocp ; use iocp as sys ; } else { compile_error ! ("polling does not support this target OS") ; } }
};
}
