// Generated macro for raw_fd_impl (module)
macro_rules! Depcrateraw_fd_impl {
() => {
// Module: crate
// Provides: {"raw_fd_impl"}
// Dependencies: {}
# [cfg (all (any (target_os = "linux" , target_os = "android" , target_os = "redox" , target_os = "illumos" , target_os = "solaris" , target_vendor = "apple" , target_os = "freebsd" , target_os = "netbsd" , target_os = "openbsd" , target_os = "dragonfly" ,) , not (polling_test_poll_backend) ,))] # [cfg_attr (docsrs , doc (cfg (any (target_os = "linux" , target_os = "android" , target_os = "redox" , target_os = "illumos" , target_os = "solaris" , target_vendor = "apple" , target_os = "freebsd" , target_os = "netbsd" , target_os = "openbsd" , target_os = "dragonfly" ,))))] mod raw_fd_impl { use crate :: Poller ; use std :: os :: unix :: io :: { AsFd , AsRawFd , BorrowedFd , RawFd } ; impl AsRawFd for Poller { fn as_raw_fd (& self) -> RawFd { self . poller . as_raw_fd () } } impl AsFd for Poller { fn as_fd (& self) -> BorrowedFd < '_ > { self . poller . as_fd () } } }
};
}
