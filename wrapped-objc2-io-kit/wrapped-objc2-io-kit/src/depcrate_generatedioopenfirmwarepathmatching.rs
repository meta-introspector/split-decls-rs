// Generated macro for IOOpenFirmwarePathMatching (function)
macro_rules! Depcrate_generatedIOOpenFirmwarePathMatching {
() => {
// Module: crate::generated
// Provides: {"IOOpenFirmwarePathMatching"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " - `path` must be a valid pointer."] # [doc = " - The returned generic must be of the correct type."] # [doc = " - The returned generic must be of the correct type."] # [cfg (feature = "libc")] # [deprecated] # [inline] pub unsafe extern "C-unwind" fn IOOpenFirmwarePathMatching (main_port : libc :: mach_port_t , options : u32 , path : * const c_char ,) -> Option < CFRetained < CFMutableDictionary > > { extern "C-unwind" { fn IOOpenFirmwarePathMatching (main_port : libc :: mach_port_t , options : u32 , path : * const c_char ,) -> Option < NonNull < CFMutableDictionary > > ; } let ret = unsafe { IOOpenFirmwarePathMatching (main_port , options , path) } ; ret . map (| ret | unsafe { CFRetained :: retain (ret) }) }
};
}
