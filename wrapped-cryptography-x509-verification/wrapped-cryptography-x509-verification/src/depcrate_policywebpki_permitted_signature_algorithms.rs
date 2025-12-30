// Generated macro for WEBPKI_PERMITTED_SIGNATURE_ALGORITHMS (static)
macro_rules! Depcrate_policyWEBPKI_PERMITTED_SIGNATURE_ALGORITHMS {
() => {
// Module: crate::policy
// Provides: {"WEBPKI_PERMITTED_SIGNATURE_ALGORITHMS"}
// Dependencies: {}
# [doc = " Permitted algorithms, from CA/B Forum's Baseline Requirements, section 7.1.3.2 (pages 96-98)"] # [doc = " https://cabforum.org/wp-content/uploads/CA-Browser-Forum-BR-v2.0.0.pdf"] pub static WEBPKI_PERMITTED_SIGNATURE_ALGORITHMS : LazyLock < Arc < HashSet < AlgorithmIdentifier < '_ > > > > = LazyLock :: new (| | { Arc :: new (HashSet :: from ([RSASSA_PKCS1V15_SHA256 . clone () , RSASSA_PKCS1V15_SHA384 . clone () , RSASSA_PKCS1V15_SHA512 . clone () , RSASSA_PSS_SHA256 . clone () , RSASSA_PSS_SHA384 . clone () , RSASSA_PSS_SHA512 . clone () , ECDSA_SHA256 . clone () , ECDSA_SHA384 . clone () , ECDSA_SHA512 . clone () ,])) }) ;
};
}
