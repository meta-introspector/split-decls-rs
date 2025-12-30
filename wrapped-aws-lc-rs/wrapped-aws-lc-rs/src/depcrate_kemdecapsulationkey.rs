// Generated macro for DecapsulationKey (struct)
macro_rules! Depcrate_kemDecapsulationKey {
() => {
// Module: crate::kem
// Provides: {"DecapsulationKey"}
// Dependencies: {}
# [doc = " A serializable decapulsation key usable with KEMs. This can be randomly generated with `DecapsulationKey::generate`."] pub struct DecapsulationKey < Id = AlgorithmId > where Id : AlgorithmIdentifier , { algorithm : & 'static Algorithm < Id > , evp_pkey : LcPtr < EVP_PKEY > , }
};
}
