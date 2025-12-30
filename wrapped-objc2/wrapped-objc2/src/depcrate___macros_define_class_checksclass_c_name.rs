// Generated macro for class_c_name (function)
macro_rules! Depcrate___macros_define_class_checksclass_c_name {
() => {
// Module: crate::__macros::define_class::checks
// Provides: {"class_c_name"}
// Dependencies: {}
# [doc = " Convert a class name with a trailing NUL byte to a `CStr`, at `const`."] # [track_caller] pub const fn class_c_name (name : & str) -> & CStr { let bytes = name . as_bytes () ; let mut i = 0 ; while i < bytes . len () - 1 { if bytes [i] == 0 { panic ! ("class name must not contain interior NUL bytes") ; } i += 1 ; } if let Ok (c_name) = CStr :: from_bytes_until_nul (bytes) { c_name } else { unreachable ! () } }
};
}
