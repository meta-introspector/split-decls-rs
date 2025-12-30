// Generated macro for Pbes2Params (struct)
macro_rules! Depcrate_pbe_paramsPbes2Params {
() => {
// Module: crate::pbe_params
// Provides: {"Pbes2Params"}
// Dependencies: {}
# [doc = "```text"] # [doc = " PBES2-params ::= SEQUENCE {"] # [doc = "      keyDerivationFunc AlgorithmIdentifier {{PBES2-KDFs}},"] # [doc = "      encryptionScheme AlgorithmIdentifier {{PBES2-Encs}} }"] # [doc = "```"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct Pbes2Params { pub kdf : AlgorithmIdentifierOwned , pub encryption : AlgorithmIdentifierOwned , }
};
}
