// Generated macro for IMAGEHLP_LINEW64 (struct)
macro_rules! Depcrate_windows_sysIMAGEHLP_LINEW64 {
() => {
// Module: crate::windows_sys
// Provides: {"IMAGEHLP_LINEW64"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy)] pub struct IMAGEHLP_LINEW64 { pub SizeOfStruct : u32 , pub Key : * mut core :: ffi :: c_void , pub LineNumber : u32 , pub FileName : PWSTR , pub Address : u64 , }
};
}
