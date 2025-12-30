// Generated macro for EncapsulationKey (struct)
macro_rules! Depcrate_kemEncapsulationKey {
() => {
// Module: crate::kem
// Provides: {"EncapsulationKey"}
// Dependencies: {}
# [doc = " A serializable encapsulation key usable with KEM algorithms. Constructed"] # [doc = " from either a `DecapsulationKey` or raw bytes."] pub struct EncapsulationKey < Id = AlgorithmId > where Id : AlgorithmIdentifier , { algorithm : & 'static Algorithm < Id > , evp_pkey : LcPtr < EVP_PKEY > , }
};
}
