// Generated macro for Algorithm (struct)
macro_rules! Depcrate_kemAlgorithm {
() => {
// Module: crate::kem
// Provides: {"Algorithm"}
// Dependencies: {}
# [doc = " A KEM algorithm"] # [derive (PartialEq)] pub struct Algorithm < Id = AlgorithmId > where Id : AlgorithmIdentifier , { pub (crate) id : Id , pub (crate) decapsulate_key_size : usize , pub (crate) encapsulate_key_size : usize , pub (crate) ciphertext_size : usize , pub (crate) shared_secret_size : usize , }
};
}
