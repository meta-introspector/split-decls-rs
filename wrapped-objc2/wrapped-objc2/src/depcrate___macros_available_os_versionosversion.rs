// Generated macro for OSVersion (struct)
macro_rules! Depcrate___macros_available_os_versionOSVersion {
() => {
// Module: crate::__macros::available::os_version
// Provides: {"OSVersion"}
// Dependencies: {}
# [doc = " The size of the fields here are limited by Mach-O's `LC_BUILD_VERSION`."] # [repr (C)] # [derive (Clone , Copy)] pub struct OSVersion { # [cfg (target_endian = "little")] pub patch : u8 , # [cfg (target_endian = "little")] pub minor : u8 , # [cfg (target_endian = "little")] pub major : u16 , # [cfg (target_endian = "big")] pub major : u16 , # [cfg (target_endian = "big")] pub minor : u8 , # [cfg (target_endian = "big")] pub patch : u8 , }
};
}
