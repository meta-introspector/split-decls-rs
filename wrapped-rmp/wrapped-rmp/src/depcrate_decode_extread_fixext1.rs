// Generated macro for read_fixext1 (function)
macro_rules! Depcrate_decode_extread_fixext1 {
() => {
// Module: crate::decode::ext
// Provides: {"read_fixext1"}
// Dependencies: {}
# [doc = " Attempts to read exactly 3 bytes from the given reader and interpret them as a fixext1 type"] # [doc = " with data attached."] # [doc = ""] # [doc = " According to the MessagePack specification, a fixext1 stores an integer and a byte array whose"] # [doc = " length is 1 byte. Its marker byte is `0xd4`."] # [doc = ""] # [doc = " Note, that this function copies a byte array from the reader to the output `u8` variable."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueReadError` on any I/O error while reading either the marker or"] # [doc = " the data."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This function will silently retry on every EINTR received from the underlying `Read` until"] # [doc = " successful read."] pub fn read_fixext1 < R : RmpRead > (rd : & mut R) -> Result < (i8 , u8) , ValueReadError < R :: Error > > { match read_marker (rd) ? { Marker :: FixExt1 => { let ty = rd . read_data_i8 () ? ; let data = rd . read_data_u8 () ? ; Ok ((ty , data)) } marker => Err (ValueReadError :: TypeMismatch (marker)) , } }
};
}
