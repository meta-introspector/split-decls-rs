// Generated macro for copy_encode (function)
macro_rules! Depcratecopy_encode {
() => {
// Module: crate
// Provides: {"copy_encode"}
// Dependencies: {}
# [doc = " Compress all data from the given source as if using a [read::XzEncoder]."] # [doc = ""] # [doc = " Compressed data will be appended to `destination`."] pub fn copy_encode < R : Read , W : Write > (source : R , mut destination : W , level : u32) -> io :: Result < () > { io :: copy (& mut read :: XzEncoder :: new (source , level) , & mut destination) ? ; Ok (()) }
};
}
