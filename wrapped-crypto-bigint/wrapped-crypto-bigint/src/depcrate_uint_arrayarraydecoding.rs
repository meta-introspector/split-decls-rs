// Generated macro for ArrayDecoding (trait)
macro_rules! Depcrate_uint_arrayArrayDecoding {
() => {
// Module: crate::uint::array
// Provides: {"ArrayDecoding"}
// Dependencies: {}
# [doc = " Support for decoding a `Array` as a big integer."] pub trait ArrayDecoding { # [doc = " Big integer which decodes a `Array`."] type Output : ArrayEncoding + Integer ; # [doc = " Deserialize from a big-endian `Array`."] fn into_uint_be (self) -> Self :: Output ; # [doc = " Deserialize from a little-endian `Array`."] fn into_uint_le (self) -> Self :: Output ; }
};
}
