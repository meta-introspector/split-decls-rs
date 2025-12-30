// Generated macro for impl_84 (impl)
macro_rules! Depcrateimpl_84 {
() => {
// Module: crate
// Provides: {"impl_84"}
// Dependencies: {}
impl compress :: CertDecompressor for ShrinkingAlgorithm { fn algorithm (& self) -> CertificateCompressionAlgorithm { CertificateCompressionAlgorithm :: Unknown (Self :: ALGORITHM) } fn decompress (& self , input : & [u8] , output : & mut [u8] ,) -> Result < () , compress :: DecompressionFailed > { if output . len () != input . len () + 2 { return Err (compress :: DecompressionFailed) ; } output [.. 2] . copy_from_slice (& [0 , 0]) ; output [2 ..] . copy_from_slice (input) ; Ok (()) } }
};
}
