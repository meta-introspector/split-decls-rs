// Generated macro for const_nonzero_i32 (function)
macro_rules! Depcrate_errorconst_nonzero_i32 {
() => {
// Module: crate::error
// Provides: {"const_nonzero_i32"}
// Dependencies: {}
# [doc = " Converts an HRESULT into a NonZeroI32. If the input is S_OK (zero), then this is converted to"] # [doc = " S_EMPTY_ERROR. This is necessary because NonZeroI32, as the name implies, cannot represent the"] # [doc = " value zero. So we remap it to a value no one should be using, during storage."] const fn const_nonzero_i32 (i : i32) -> NonZeroI32 { if let Some (nz) = NonZeroI32 :: new (i) { nz } else { panic ! () ; } }
};
}
