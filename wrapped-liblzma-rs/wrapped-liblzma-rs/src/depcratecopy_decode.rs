// Generated macro for copy_decode (function)
macro_rules! Depcratecopy_decode {
() => {
// Module: crate
// Provides: {"copy_decode"}
// Dependencies: {}
# [doc = " Decompress all data from the given source as if using a [read::XzDecoder]."] # [doc = ""] # [doc = " Decompressed data will be appended to `destination`."] pub fn copy_decode < R : Read , W : Write > (source : R , mut destination : W) -> io :: Result < () > { io :: copy (& mut read :: XzDecoder :: new (source) , & mut destination) ? ; Ok (()) }
};
}
