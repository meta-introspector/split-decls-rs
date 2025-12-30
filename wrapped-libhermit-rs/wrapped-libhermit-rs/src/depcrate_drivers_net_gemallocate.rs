// Generated macro for allocate (function)
macro_rules! Depcrate_drivers_net_gemallocate {
() => {
// Module: crate::drivers::net::gem
// Provides: {"allocate"}
// Dependencies: {}
# [doc = " Soft-deprecated in favor of `DeviceAlloc`"] fn allocate (size : usize , no_execution : bool) -> VirtAddr { let layout = Layout :: from_size_align (size , 8) . unwrap () ; let allocation = DeviceAlloc . allocate (layout) . unwrap () ; VirtAddr :: from_ptr (allocation . as_ptr ()) }
};
}
