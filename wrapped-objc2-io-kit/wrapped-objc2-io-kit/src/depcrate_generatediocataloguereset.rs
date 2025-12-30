// Generated macro for IOCatalogueReset (function)
macro_rules! Depcrate_generatedIOCatalogueReset {
() => {
// Module: crate::generated
// Provides: {"IOCatalogueReset"}
// Dependencies: {}
# [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOCatalogueReset (main_port : libc :: mach_port_t , flag : u32 ,) -> libc :: kern_return_t { extern "C-unwind" { fn IOCatalogueReset (main_port : libc :: mach_port_t , flag : u32) -> libc :: kern_return_t ; } unsafe { IOCatalogueReset (main_port , flag) } }
};
}
