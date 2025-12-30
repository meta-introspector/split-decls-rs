// Generated macro for impl_283 (impl)
macro_rules! Depcrateimpl_283 {
() => {
// Module: crate
// Provides: {"impl_283"}
// Dependencies: {}
# [cfg (not (feature = "unstable-test"))] impl IdRanges { pub fn get () -> Self { extern "C" { static __DEFMT_MARKER_TRACE_START : u8 ; static __DEFMT_MARKER_TRACE_END : u8 ; static __DEFMT_MARKER_DEBUG_START : u8 ; static __DEFMT_MARKER_DEBUG_END : u8 ; static __DEFMT_MARKER_INFO_START : u8 ; static __DEFMT_MARKER_INFO_END : u8 ; static __DEFMT_MARKER_WARN_START : u8 ; static __DEFMT_MARKER_WARN_END : u8 ; static __DEFMT_MARKER_ERROR_START : u8 ; static __DEFMT_MARKER_ERROR_END : u8 ; } let trace_start = unsafe { & __DEFMT_MARKER_TRACE_START as * const u8 as u16 } ; let trace_end = unsafe { & __DEFMT_MARKER_TRACE_END as * const u8 as u16 } ; let debug_start = unsafe { & __DEFMT_MARKER_DEBUG_START as * const u8 as u16 } ; let debug_end = unsafe { & __DEFMT_MARKER_DEBUG_END as * const u8 as u16 } ; let info_start = unsafe { & __DEFMT_MARKER_INFO_START as * const u8 as u16 } ; let info_end = unsafe { & __DEFMT_MARKER_INFO_END as * const u8 as u16 } ; let warn_start = unsafe { & __DEFMT_MARKER_WARN_START as * const u8 as u16 } ; let warn_end = unsafe { & __DEFMT_MARKER_WARN_END as * const u8 as u16 } ; let error_start = unsafe { & __DEFMT_MARKER_ERROR_START as * const u8 as u16 } ; let error_end = unsafe { & __DEFMT_MARKER_ERROR_END as * const u8 as u16 } ; Self { trace : trace_start .. trace_end , debug : debug_start .. debug_end , info : info_start .. info_end , warn : warn_start .. warn_end , error : error_start .. error_end , } } }
};
}
