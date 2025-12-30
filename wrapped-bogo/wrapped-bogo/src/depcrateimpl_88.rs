// Generated macro for impl_88 (impl)
macro_rules! Depcrateimpl_88 {
() => {
// Module: crate
// Provides: {"impl_88"}
// Dependencies: {}
impl compress :: CertCompressor for ExpandingAlgorithm { fn algorithm (& self) -> CertificateCompressionAlgorithm { CertificateCompressionAlgorithm :: Unknown (0xff02) } fn compress (& self , mut input : Vec < u8 > , _ : compress :: CompressionLevel ,) -> Result < Vec < u8 > , compress :: CompressionFailed > { input . insert (0 , 1) ; input . insert (1 , 2) ; input . insert (2 , 3) ; input . insert (3 , 4) ; Ok (input) } }
};
}
