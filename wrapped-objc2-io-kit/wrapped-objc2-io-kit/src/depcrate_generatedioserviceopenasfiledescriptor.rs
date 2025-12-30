// Generated macro for IOServiceOpenAsFileDescriptor (function)
macro_rules! Depcrate_generatedIOServiceOpenAsFileDescriptor {
() => {
// Module: crate::generated
// Provides: {"IOServiceOpenAsFileDescriptor"}
// Dependencies: {}
# [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOServiceOpenAsFileDescriptor (service : io_service_t , oflag : c_int ,) -> c_int { extern "C-unwind" { fn IOServiceOpenAsFileDescriptor (service : io_service_t , oflag : c_int) -> c_int ; } unsafe { IOServiceOpenAsFileDescriptor (service , oflag) } }
};
}
