// Generated macro for impl_13 (impl)
macro_rules! Depcrate_valueimpl_13 {
() => {
// Module: crate::value
// Provides: {"impl_13"}
// Dependencies: {}
impl JsonStr { # [doc = "\n    Treat a string as native JSON.\n    "] pub const fn new < 'a > (json : & 'a str) -> & 'a Self { unsafe { & * (json as * const _ as * const JsonStr) } } # [doc = "\n    Get a reference to the underlying string.\n    "] pub const fn as_str (& self) -> & str { & self . 0 } # [doc = "\n    Get a reference to the bytes of the underlying string.\n    "] pub const fn as_bytes (& self) -> & [u8] { self . 0 . as_bytes () } }
};
}
