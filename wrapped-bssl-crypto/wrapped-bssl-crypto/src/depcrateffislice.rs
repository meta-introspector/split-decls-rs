// Generated macro for FfiSlice (trait)
macro_rules! DepcrateFfiSlice {
() => {
// Module: crate
// Provides: {"FfiSlice"}
// Dependencies: {}
# [doc = " FfiSlice exists to provide `as_ffi_ptr` on slices. Calling `as_ptr` on an"] # [doc = " empty Rust slice may return the alignment of the type, rather than NULL, as"] # [doc = " the pointer. When passing pointers into C/C++ code, that is not a valid"] # [doc = " pointer. Thus this method should be used whenever passing a pointer to a"] # [doc = " slice into BoringSSL code."] trait FfiSlice < T > { fn as_ffi_ptr (& self) -> * const T ; fn as_ffi_void_ptr (& self) -> * const c_void { self . as_ffi_ptr () as * const c_void } }
};
}
