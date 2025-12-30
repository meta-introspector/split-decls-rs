// Generated macro for impl_76 (impl)
macro_rules! Depcrate_watchesimpl_76 {
() => {
// Module: crate::watches
// Provides: {"impl_76"}
// Dependencies: {}
impl PartialEq for WatchDescriptor { fn eq (& self , other : & Self) -> bool { let self_fd = self . fd . upgrade () ; let other_fd = other . fd . upgrade () ; self . id == other . id && self_fd . is_some () && self_fd == other_fd } }
};
}
