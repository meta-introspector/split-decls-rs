// Generated macro for PGET_MODULE_BASE_ROUTINE64 (type)
macro_rules! Depcrate_windows_sysPGET_MODULE_BASE_ROUTINE64 {
() => {
// Module: crate::windows_sys
// Provides: {"PGET_MODULE_BASE_ROUTINE64"}
// Dependencies: {}
pub type PGET_MODULE_BASE_ROUTINE64 = Option < unsafe extern "system" fn (hprocess : HANDLE , address : u64) -> u64 > ;
};
}
