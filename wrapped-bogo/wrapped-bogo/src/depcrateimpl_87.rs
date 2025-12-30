// Generated macro for impl_87 (impl)
macro_rules! Depcrateimpl_87 {
() => {
// Module: crate
// Provides: {"impl_87"}
// Dependencies: {}
impl compress :: CertDecompressor for ExpandingAlgorithm { fn algorithm (& self) -> CertificateCompressionAlgorithm { CertificateCompressionAlgorithm :: Unknown (0xff02) } fn decompress (& self , input : & [u8] , output : & mut [u8] ,) -> Result < () , compress :: DecompressionFailed > { if output . len () + 4 != input . len () { return Err (compress :: DecompressionFailed) ; } if input [.. 4] != [1 , 2 , 3 , 4] { return Err (compress :: DecompressionFailed) ; } output . copy_from_slice (& input [4 ..]) ; Ok (()) } }
};
}
