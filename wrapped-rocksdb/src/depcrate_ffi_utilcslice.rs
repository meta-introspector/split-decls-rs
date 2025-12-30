// Generated macro for CSlice (struct)
macro_rules! Depcrate_ffi_utilCSlice {
() => {
// Module: crate::ffi_util
// Provides: {"CSlice"}
// Dependencies: {}
# [doc = " Owned malloc-allocated memory slice."] # [doc = " Do not derive `Clone` for this because it will cause double-free."] pub struct CSlice { data : * const c_char , len : size_t , }
};
}
