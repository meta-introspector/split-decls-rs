// Generated macro for impl_90 (impl)
macro_rules! Depcrateimpl_90 {
() => {
// Module: crate
// Provides: {"impl_90"}
// Dependencies: {}
impl compress :: CertDecompressor for RandomAlgorithm { fn algorithm (& self) -> CertificateCompressionAlgorithm { CertificateCompressionAlgorithm :: Unknown (0xff03) } fn decompress (& self , input : & [u8] , output : & mut [u8] ,) -> Result < () , compress :: DecompressionFailed > { if output . len () + 1 != input . len () { return Err (compress :: DecompressionFailed) ; } output . copy_from_slice (& input [1 ..]) ; Ok (()) } }
};
}
