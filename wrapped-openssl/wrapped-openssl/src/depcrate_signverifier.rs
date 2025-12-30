// Generated macro for Verifier (struct)
macro_rules! Depcrate_signVerifier {
() => {
// Module: crate::sign
// Provides: {"Verifier"}
// Dependencies: {}
# [doc = " A type which can be used to verify the integrity and authenticity"] # [doc = " of data given the signature."] pub struct Verifier < 'a > { md_ctx : * mut ffi :: EVP_MD_CTX , pctx : * mut ffi :: EVP_PKEY_CTX , pkey_pd : PhantomData < & 'a () > , }
};
}
