macro_rules! is_version_compatible {
    () => {
        unsafe fn is_version_compatible (version : * const c_char , stream_size : i32) -> bool { let Some (expected_major_version) = (unsafe { version . as_ref () }) else { return false ; } ; if * expected_major_version as u8 != LIBZ_RS_SYS_VERSION . as_bytes () [0] { return false ; } core :: mem :: size_of :: < z_stream > () as i32 == stream_size }
    };
}

is_version_compatible!()