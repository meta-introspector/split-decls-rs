// Generated macro for AltBn128CompressionError (enum)
macro_rules! Depcrate_compressionAltBn128CompressionError {
() => {
// Module: crate::compression
// Provides: {"AltBn128CompressionError"}
// Dependencies: {}
# [derive (Debug , Error , Clone , PartialEq , Eq)] pub enum AltBn128CompressionError { # [error ("Unexpected error")] UnexpectedError , # [error ("Failed to decompress g1")] G1DecompressionFailed , # [error ("Failed to decompress g2")] G2DecompressionFailed , # [error ("Failed to compress affine g1")] G1CompressionFailed , # [error ("Failed to compress affine g2")] G2CompressionFailed , # [error ("Invalid input size")] InvalidInputSize , }
};
}
