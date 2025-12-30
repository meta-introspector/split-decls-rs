// Generated macro for read_marker (function)
macro_rules! Depcrate_decoderead_marker {
() => {
// Module: crate::decode
// Provides: {"read_marker"}
// Dependencies: {}
# [doc = " Attempts to read a single byte from the given reader and to decode it as a MessagePack marker."] # [inline] pub fn read_marker < R : RmpRead > (rd : & mut R) -> Result < Marker , MarkerReadError < R :: Error > > { Ok (Marker :: from_u8 (rd . read_u8 () ?)) }
};
}
