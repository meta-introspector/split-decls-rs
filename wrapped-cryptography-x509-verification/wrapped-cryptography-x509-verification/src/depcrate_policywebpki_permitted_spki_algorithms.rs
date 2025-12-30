// Generated macro for WEBPKI_PERMITTED_SPKI_ALGORITHMS (static)
macro_rules! Depcrate_policyWEBPKI_PERMITTED_SPKI_ALGORITHMS {
() => {
// Module: crate::policy
// Provides: {"WEBPKI_PERMITTED_SPKI_ALGORITHMS"}
// Dependencies: {}
# [doc = " Permitted algorithms, from CA/B Forum's Baseline Requirements, section 7.1.3.1 (page 96)"] # [doc = " https://cabforum.org/wp-content/uploads/CA-Browser-Forum-BR-v2.0.0.pdf"] pub static WEBPKI_PERMITTED_SPKI_ALGORITHMS : LazyLock < Arc < HashSet < AlgorithmIdentifier < '_ > > > > = LazyLock :: new (| | { Arc :: new (HashSet :: from ([SPKI_RSA . clone () , SPKI_SECP256R1 . clone () , SPKI_SECP384R1 . clone () , SPKI_SECP521R1 . clone () ,])) }) ;
};
}
