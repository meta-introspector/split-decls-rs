// Generated macro for impl_790 (impl)
macro_rules! Depcrate_rsaimpl_790 {
() => {
// Module: crate::rsa
// Provides: {"impl_790"}
// Dependencies: {}
impl Padding { pub const NONE : Padding = Padding (ffi :: RSA_NO_PADDING) ; pub const PKCS1 : Padding = Padding (ffi :: RSA_PKCS1_PADDING) ; pub const PKCS1_OAEP : Padding = Padding (ffi :: RSA_PKCS1_OAEP_PADDING) ; pub const PKCS1_PSS : Padding = Padding (ffi :: RSA_PKCS1_PSS_PADDING) ; # [doc = " Creates a `Padding` from an integer representation."] pub fn from_raw (value : c_int) -> Padding { Padding (value) } # [doc = " Returns the integer representation of `Padding`."] # [allow (clippy :: trivially_copy_pass_by_ref)] pub fn as_raw (& self) -> c_int { self . 0 } }
};
}
