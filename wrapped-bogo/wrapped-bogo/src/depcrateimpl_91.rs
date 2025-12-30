// Generated macro for impl_91 (impl)
macro_rules! Depcrateimpl_91 {
() => {
// Module: crate
// Provides: {"impl_91"}
// Dependencies: {}
impl compress :: CertCompressor for RandomAlgorithm { fn algorithm (& self) -> CertificateCompressionAlgorithm { CertificateCompressionAlgorithm :: Unknown (0xff03) } fn compress (& self , mut input : Vec < u8 > , _ : compress :: CompressionLevel ,) -> Result < Vec < u8 > , compress :: CompressionFailed > { let random_byte = { let mut bytes = [0] ; ring :: DEFAULT_PROVIDER . secure_random . fill (& mut bytes) . unwrap () ; bytes [0] } ; input . insert (0 , random_byte) ; Ok (input) } }
};
}
