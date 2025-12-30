// Generated macro for serial_buf_hypercall (function)
macro_rules! Depcrate_syscalls_interfaces_uhyveserial_buf_hypercall {
() => {
// Module: crate::syscalls::interfaces::uhyve
// Provides: {"serial_buf_hypercall"}
// Dependencies: {}
# [doc = " perform a SerialWriteBuffer hypercall with `buf` as payload."] # [inline] # [cfg_attr (target_arch = "riscv64" , expect (dead_code))] pub (crate) fn serial_buf_hypercall (buf : & [u8]) { let p = SerialWriteBufferParams { buf : virtual_to_physical (VirtAddr :: from_ptr (core :: ptr :: from_ref :: < [u8] > (buf))) . unwrap () , len : buf . len () , } ; uhyve_hypercall (Hypercall :: SerialWriteBuffer (& p)) ; }
};
}
