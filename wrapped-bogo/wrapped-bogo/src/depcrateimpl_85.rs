// Generated macro for impl_85 (impl)
macro_rules! Depcrateimpl_85 {
() => {
// Module: crate
// Provides: {"impl_85"}
// Dependencies: {}
impl compress :: CertCompressor for ShrinkingAlgorithm { fn algorithm (& self) -> CertificateCompressionAlgorithm { CertificateCompressionAlgorithm :: Unknown (Self :: ALGORITHM) } fn compress (& self , mut input : Vec < u8 > , _ : compress :: CompressionLevel ,) -> Result < Vec < u8 > , compress :: CompressionFailed > { assert_eq ! (input [.. 2] , [0 , 0]) ; input . drain (0 .. 2) ; Ok (input) } }
};
}
