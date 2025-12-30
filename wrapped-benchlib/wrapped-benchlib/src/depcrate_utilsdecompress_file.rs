// Generated macro for decompress_file (function)
macro_rules! Depcrate_utilsdecompress_file {
() => {
// Module: crate::utils
// Provides: {"decompress_file"}
// Dependencies: {}
# [doc = " Decompresses the contents of a gzipped (.gz) file."] # [cfg (feature = "compression")] pub fn decompress_file (contents : & [u8]) -> Vec < u8 > { use std :: io :: Read ; let mut tar = flate2 :: bufread :: GzDecoder :: new (contents) ; let mut result = Vec :: new () ; tar . read_to_end (& mut result) . expect ("Cannot decompress file") ; result }
};
}
