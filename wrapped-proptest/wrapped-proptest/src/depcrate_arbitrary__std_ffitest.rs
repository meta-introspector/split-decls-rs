// Generated macro for test (module)
macro_rules! Depcrate_arbitrary__std_ffitest {
() => {
// Module: crate::arbitrary::_std::ffi
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { no_panic_test ! (c_string => CString , os_string => OsString , box_c_str => Box < CStr >, box_os_str => Box < OsStr >, into_string_error => IntoStringError , from_bytes_with_nul => FromBytesWithNulError) ; # [cfg (feature = "unstable")] no_panic_test ! (rc_c_str => Rc < CStr >, rc_os_str => Rc < OsStr >, arc_c_str => Arc < CStr >, arc_os_str => Arc < OsStr >) ; }
};
}
