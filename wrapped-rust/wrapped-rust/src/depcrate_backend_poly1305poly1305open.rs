// Generated macro for Poly1305Open (struct)
macro_rules! Depcrate_backend_poly1305Poly1305Open {
() => {
// Module: crate::backend::poly1305
// Provides: {"Poly1305Open"}
// Dependencies: {}
# [cfg (not (any (CRYPTOGRAPHY_IS_LIBRESSL , CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC)))] struct Poly1305Open { signer : openssl :: sign :: Signer < 'static > , }
};
}
