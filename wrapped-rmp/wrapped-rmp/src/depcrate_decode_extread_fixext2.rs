// Generated macro for read_fixext2 (function)
macro_rules! Depcrate_decode_extread_fixext2 {
() => {
// Module: crate::decode::ext
// Provides: {"read_fixext2"}
// Dependencies: {}
# [doc = " Attempts to read exactly 4 bytes from the given reader and interpret them as a fixext2 type"] # [doc = " with data attached."] # [doc = ""] # [doc = " According to the MessagePack specification, a fixext2 stores an integer and a byte array whose"] # [doc = " length is 2 bytes. Its marker byte is `0xd5`."] # [doc = ""] # [doc = " Note, that this function copies a byte array from the reader to the output buffer, which is"] # [doc = " unlikely if you want zero-copy functionality."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueReadError` on any I/O error while reading either the marker or"] # [doc = " the data."] pub fn read_fixext2 < R : RmpRead > (rd : & mut R) -> Result < (i8 , [u8 ; 2]) , ValueReadError < R :: Error > > { match read_marker (rd) ? { Marker :: FixExt2 => { let mut buf = [0 ; 2] ; read_fixext_data (rd , & mut buf) . map (| ty | (ty , buf)) } marker => Err (ValueReadError :: TypeMismatch (marker)) , } }
};
}
