// Generated macro for impl_746 (impl)
macro_rules! Depcrate_pkey_ctximpl_746 {
() => {
// Module: crate::pkey_ctx
// Provides: {"impl_746"}
// Dependencies: {}
# [cfg (any (ossl111 , libressl360))] impl HkdfMode { # [doc = " This is the default mode. Calling [`derive`][PkeyCtxRef::derive] on a [`PkeyCtxRef`] set up"] # [doc = " for HKDF will perform an extract followed by an expand operation in one go. The derived key"] # [doc = " returned will be the result after the expand operation. The intermediate fixed-length"] # [doc = " pseudorandom key K is not returned."] pub const EXTRACT_THEN_EXPAND : Self = HkdfMode (ffi :: EVP_PKEY_HKDEF_MODE_EXTRACT_AND_EXPAND) ; # [doc = " In this mode calling [`derive`][PkeyCtxRef::derive] will just perform the extract operation."] # [doc = " The value returned will be the intermediate fixed-length pseudorandom key K."] # [doc = ""] # [doc = " The digest, key and salt values must be set before a key is derived or an error occurs."] pub const EXTRACT_ONLY : Self = HkdfMode (ffi :: EVP_PKEY_HKDEF_MODE_EXTRACT_ONLY) ; # [doc = " In this mode calling [`derive`][PkeyCtxRef::derive] will just perform the expand operation."] # [doc = " The input key should be set to the intermediate fixed-length pseudorandom key K returned"] # [doc = " from a previous extract operation."] # [doc = ""] # [doc = " The digest, key and info values must be set before a key is derived or an error occurs."] pub const EXPAND_ONLY : Self = HkdfMode (ffi :: EVP_PKEY_HKDEF_MODE_EXPAND_ONLY) ; }
};
}
