// Generated macro for data_addr (function)
macro_rules! Depcrate_syscalls_interfaces_uhyvedata_addr {
() => {
// Module: crate::syscalls::interfaces::uhyve
// Provides: {"data_addr"}
// Dependencies: {}
# [doc = " calculates the physical address of the struct passed as reference."] # [inline] fn data_addr < T > (data : & T) -> u64 { paging :: virtual_to_physical (VirtAddr :: from_ptr (ptr :: from_ref (data))) . unwrap () . as_u64 () }
};
}
