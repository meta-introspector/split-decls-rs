// Generated macro for kqueue (module)
macro_rules! Depcrate_oskqueue {
() => {
// Module: crate::os
// Provides: {"kqueue"}
// Dependencies: {}
# [cfg (all (any (target_vendor = "apple" , target_os = "freebsd" , target_os = "netbsd" , target_os = "openbsd" , target_os = "dragonfly" ,) , not (polling_test_poll_backend) ,))] pub mod kqueue ;
};
}
