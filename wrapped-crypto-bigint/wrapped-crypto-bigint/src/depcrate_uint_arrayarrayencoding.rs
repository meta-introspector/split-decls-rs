// Generated macro for ArrayEncoding (trait)
macro_rules! Depcrate_uint_arrayArrayEncoding {
() => {
// Module: crate::uint::array
// Provides: {"ArrayEncoding"}
// Dependencies: {}
# [doc = " Support for encoding a big integer as a `Array`."] pub trait ArrayEncoding : Encoding { # [doc = " Size of a byte array which encodes a big integer."] type ByteSize : ArraySize + Add + Eq + Ord + Unsigned ; # [doc = " Deserialize from a big-endian byte array."] fn from_be_byte_array (bytes : ByteArray < Self >) -> Self ; # [doc = " Deserialize from a little-endian byte array."] fn from_le_byte_array (bytes : ByteArray < Self >) -> Self ; # [doc = " Serialize to a big-endian byte array."] fn to_be_byte_array (& self) -> ByteArray < Self > ; # [doc = " Serialize to a little-endian byte array."] fn to_le_byte_array (& self) -> ByteArray < Self > ; }
};
}
