// Generated macro for determine_endianness (function)
macro_rules! Depcrate_binarydetermine_endianness {
() => {
// Module: crate::binary
// Provides: {"determine_endianness"}
// Dependencies: {}
# [doc = " Gets the endianness of a binary resource bundle's data."] pub fn determine_endianness (resb : & [u8]) -> Result < Endianness , BinaryDeserializerError > { let header = BinHeader :: try_from (resb) ? ; Ok (header . repr_info . endianness) }
};
}
