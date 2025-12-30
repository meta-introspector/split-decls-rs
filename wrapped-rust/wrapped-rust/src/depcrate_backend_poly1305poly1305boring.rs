// Generated macro for Poly1305Boring (struct)
macro_rules! Depcrate_backend_poly1305Poly1305Boring {
() => {
// Module: crate::backend::poly1305
// Provides: {"Poly1305Boring"}
// Dependencies: {}
# [cfg (any (CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_LIBRESSL , CRYPTOGRAPHY_IS_AWSLC))] struct Poly1305Boring { context : cryptography_openssl :: poly1305 :: Poly1305State , }
};
}
