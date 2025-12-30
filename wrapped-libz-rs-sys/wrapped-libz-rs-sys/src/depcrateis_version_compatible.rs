// Generated macro for is_version_compatible (function)
macro_rules! Depcrateis_version_compatible {
() => {
// Module: crate
// Provides: {"is_version_compatible"}
// Dependencies: {}
unsafe fn is_version_compatible (version : * const c_char , stream_size : i32) -> bool { let Some (expected_major_version) = (unsafe { version . as_ref () }) else { return false ; } ; if * expected_major_version as u8 != LIBZ_RS_SYS_VERSION . as_bytes () [0] { return false ; } core :: mem :: size_of :: < z_stream > () as i32 == stream_size }
};
}
