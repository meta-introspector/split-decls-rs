// Generated macro for impl_175 (impl)
macro_rules! Depcrate_traitsimpl_175 {
() => {
// Module: crate::traits
// Provides: {"impl_175"}
// Dependencies: {}
unsafe impl < A , R > ManualBlockEncoding for NoBlockEncoding < A , R > where A : EncodeArguments , R : EncodeReturn , { type Arguments = A ; type Return = R ; const ENCODING_CSTR : & 'static CStr = unsafe { CStr :: from_bytes_with_nul_unchecked (b"\0") } ; }
};
}
