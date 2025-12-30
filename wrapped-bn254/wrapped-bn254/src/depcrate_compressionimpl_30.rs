// Generated macro for impl_30 (impl)
macro_rules! Depcrate_compressionimpl_30 {
() => {
// Module: crate::compression
// Provides: {"impl_30"}
// Dependencies: {}
impl From < AltBn128CompressionError > for u64 { fn from (v : AltBn128CompressionError) -> u64 { match v { AltBn128CompressionError :: G1DecompressionFailed => 1 , AltBn128CompressionError :: G2DecompressionFailed => 2 , AltBn128CompressionError :: G1CompressionFailed => 3 , AltBn128CompressionError :: G2CompressionFailed => 4 , AltBn128CompressionError :: InvalidInputSize => 5 , AltBn128CompressionError :: UnexpectedError => 6 , } } }
};
}
