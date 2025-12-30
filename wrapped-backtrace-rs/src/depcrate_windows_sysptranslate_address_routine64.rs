// Generated macro for PTRANSLATE_ADDRESS_ROUTINE64 (type)
macro_rules! Depcrate_windows_sysPTRANSLATE_ADDRESS_ROUTINE64 {
() => {
// Module: crate::windows_sys
// Provides: {"PTRANSLATE_ADDRESS_ROUTINE64"}
// Dependencies: {}
pub type PTRANSLATE_ADDRESS_ROUTINE64 = Option < unsafe extern "system" fn (hprocess : HANDLE , hthread : HANDLE , lpaddr : * const ADDRESS64) -> u64 , > ;
};
}
