// Generated macro for CSlice (struct)
macro_rules! DepcrateCSlice {
() => {
// Module: crate
// Provides: {"CSlice"}
// Dependencies: {}
# [doc = " This is a helper struct which provides functions for passing slices over FFI."] # [doc = ""] # [doc = " Deprecated: use `FfiSlice` which adds less noise and lets one grep for `as_ptr`"] # [doc = " as a sign of something to check."] struct CSlice < 'a > (& 'a [u8]) ;
};
}
