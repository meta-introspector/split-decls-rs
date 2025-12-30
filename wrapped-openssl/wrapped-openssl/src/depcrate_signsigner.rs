// Generated macro for Signer (struct)
macro_rules! Depcrate_signSigner {
() => {
// Module: crate::sign
// Provides: {"Signer"}
// Dependencies: {}
# [doc = " A type which computes cryptographic signatures of data."] pub struct Signer < 'a > { md_ctx : * mut ffi :: EVP_MD_CTX , pctx : * mut ffi :: EVP_PKEY_CTX , _p : PhantomData < & 'a () > , }
};
}
