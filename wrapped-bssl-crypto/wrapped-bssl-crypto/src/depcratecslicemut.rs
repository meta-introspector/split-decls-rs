// Generated macro for CSliceMut (struct)
macro_rules! DepcrateCSliceMut {
() => {
// Module: crate
// Provides: {"CSliceMut"}
// Dependencies: {}
# [doc = " This is a helper struct which provides functions for passing mutable slices over FFI."] # [doc = ""] # [doc = " Deprecated: use `FfiMutSlice` which adds less noise and lets one grep for"] # [doc = " `as_ptr` as a sign of something to check."] struct CSliceMut < 'a > (& 'a mut [u8]) ;
};
}
