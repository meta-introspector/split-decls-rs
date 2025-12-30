// Generated macro for impl_81 (impl)
macro_rules! Depcrateimpl_81 {
() => {
// Module: crate
// Provides: {"impl_81"}
// Dependencies: {}
impl TryFrom < AlgorithmIdentifierRef < '_ > > for EncryptionScheme { type Error = der :: Error ; fn try_from (alg : AlgorithmIdentifierRef < '_ >) -> der :: Result < EncryptionScheme > { if alg . oid == pbes2 :: PBES2_OID { match alg . parameters { Some (params) => pbes2 :: Parameters :: try_from (params) . map (Into :: into) , None => Err (Tag :: OctetString . value_error () . into ()) , } } else { pbes1 :: Algorithm :: try_from (alg) . map (Into :: into) } } }
};
}
