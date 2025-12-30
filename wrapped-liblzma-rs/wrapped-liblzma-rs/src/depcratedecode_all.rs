// Generated macro for decode_all (function)
macro_rules! Depcratedecode_all {
() => {
// Module: crate
// Provides: {"decode_all"}
// Dependencies: {}
# [doc = " Decompress from the given source as if using a [read::XzDecoder]."] # [doc = ""] # [doc = " Result will be in the xz format."] pub fn decode_all < R : Read > (source : R) -> io :: Result < Vec < u8 > > { let mut vec = Vec :: new () ; let mut r = read :: XzDecoder :: new (source) ; r . read_to_end (& mut vec) ? ; Ok (vec) }
};
}
