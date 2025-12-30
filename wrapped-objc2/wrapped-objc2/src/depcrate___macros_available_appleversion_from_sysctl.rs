// Generated macro for version_from_sysctl (function)
macro_rules! Depcrate___macros_available_appleversion_from_sysctl {
() => {
// Module: crate::__macros::available::apple
// Provides: {"version_from_sysctl"}
// Dependencies: {}
# [doc = " Read the version from `kern.osproductversion` or `kern.iossupportversion`."] # [doc = ""] # [doc = " This is faster than `version_from_plist`, since it doesn't need to invoke `dlsym`."] fn version_from_sysctl () -> Option < OSVersion > { if cfg ! (target_simulator) { return None ; } extern "C" { fn sysctlbyname (name : * const c_char , oldp : * mut c_void , oldlenp : * mut usize , newp : * mut c_void , newlen : usize ,) -> c_uint ; } let sysctl_version = | name : & [u8] | { let mut buf : [u8 ; 32] = [0 ; 32] ; let mut size = buf . len () ; let ptr = buf . as_mut_ptr () . cast () ; let ret = unsafe { sysctlbyname (name . as_ptr () . cast () , ptr , & mut size , null_mut () , 0) } ; if ret != 0 { return None ; } let buf = & buf [.. (size - 1)] ; if buf . is_empty () { return None ; } Some (OSVersion :: from_bytes (buf)) } ; if cfg ! (target_os = "ios") { if let Some (ios_support_version) = sysctl_version (b"kern.iossupportversion\0") { return Some (ios_support_version) ; } if cfg ! (target_abi_macabi) { return None ; } } sysctl_version (b"kern.osproductversion\0") }
};
}
