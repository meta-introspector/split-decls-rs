// Generated macro for other_4872 (other)
macro_rules! Depcrate_generatedother_4872 {
() => {
// Module: crate::generated
// Provides: {"other_4872"}
// Dependencies: {}
extern "C-unwind" { # [doc = " # Safety"] # [doc = ""] # [doc = " - `open_firmware_path` must be a valid pointer."] # [doc = " - `bsd_name` must be a valid pointer."] # [cfg (feature = "libc")] # [deprecated] pub fn IOServiceOFPathToBSDName (main_port : libc :: mach_port_t , open_firmware_path : * mut io_name_t , bsd_name : * mut io_name_t ,) -> libc :: kern_return_t ; }
};
}
