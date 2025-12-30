// Generated macro for encode_all (function)
macro_rules! Depcrateencode_all {
() => {
// Module: crate
// Provides: {"encode_all"}
// Dependencies: {}
# [doc = " Compress from the given source as if using a [read::XzEncoder]."] # [doc = ""] # [doc = " The input data must be in the xz format."] pub fn encode_all < R : Read > (source : R , level : u32) -> io :: Result < Vec < u8 > > { let mut vec = Vec :: new () ; let mut r = read :: XzEncoder :: new (source , level) ; r . read_to_end (& mut vec) ? ; Ok (vec) }
};
}
