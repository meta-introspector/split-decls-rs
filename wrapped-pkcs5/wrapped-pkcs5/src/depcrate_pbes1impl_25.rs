// Generated macro for impl_25 (impl)
macro_rules! Depcrate_pbes1impl_25 {
() => {
// Module: crate::pbes1
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'a > TryFrom < AlgorithmIdentifierRef < 'a > > for Algorithm { type Error = der :: Error ; fn try_from (alg : AlgorithmIdentifierRef < 'a >) -> der :: Result < Self > { let encryption = EncryptionScheme :: try_from (alg . oid) . map_err (| _ | Tag :: ObjectIdentifier . value_error ()) ? ; let parameters = alg . parameters . ok_or_else (| | Tag :: OctetString . value_error ()) ? . try_into () ? ; Ok (Self { encryption , parameters , }) } }
};
}
