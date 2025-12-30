// Generated macro for impl_29 (impl)
macro_rules! Depcrate_compressionimpl_29 {
() => {
// Module: crate::compression
// Provides: {"impl_29"}
// Dependencies: {}
impl From < u64 > for AltBn128CompressionError { fn from (v : u64) -> AltBn128CompressionError { match v { 1 => AltBn128CompressionError :: G1DecompressionFailed , 2 => AltBn128CompressionError :: G2DecompressionFailed , 3 => AltBn128CompressionError :: G1CompressionFailed , 4 => AltBn128CompressionError :: G2CompressionFailed , 5 => AltBn128CompressionError :: InvalidInputSize , _ => AltBn128CompressionError :: UnexpectedError , } } }
};
}
