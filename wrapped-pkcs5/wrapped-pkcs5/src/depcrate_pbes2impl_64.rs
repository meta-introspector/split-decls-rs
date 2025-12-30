// Generated macro for impl_64 (impl)
macro_rules! Depcrate_pbes2impl_64 {
() => {
// Module: crate::pbes2
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'a > Decode < 'a > for EncryptionScheme { type Error = der :: Error ; fn decode < R : Reader < 'a > > (reader : & mut R) -> der :: Result < Self > { AlgorithmIdentifierRef :: decode (reader) . and_then (TryInto :: try_into) } }
};
}
