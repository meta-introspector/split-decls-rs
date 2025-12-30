// Generated macro for read_map_len (function)
macro_rules! Depcrate_decoderead_map_len {
() => {
// Module: crate::decode
// Provides: {"read_map_len"}
// Dependencies: {}
# [doc = " Attempts to read up to 5 bytes from the given reader and to decode them as a big-endian u32"] # [doc = " map size."] # [doc = ""] # [doc = " Map format family stores a sequence of elements in 1, 3, or 5 bytes of extra bytes in addition"] # [doc = " to the elements."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This function will silently retry on every EINTR received from the underlying `Read` until"] # [doc = " successful read."] pub fn read_map_len < R : RmpRead > (rd : & mut R) -> Result < u32 , ValueReadError < R :: Error > > { let marker = read_marker (rd) ? ; marker_to_len (rd , marker) }
};
}
